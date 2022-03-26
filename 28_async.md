# Async

Asynchronous programming is a _concurrent programming model_ allowing executing
many concurrent tasks on a small number of OS threads.

Rust's implementation of async is:

- **zero-cost** - can be performed without heap allocations and dynamic dispatch
- **lazy** - progress is made only when polled
- **zero-runtime** - runtime implementation is provided by community crates
- **single- and multi-threaded** - depending on implementation

Asynchronous tasks are defined in `async fn` functions and `async` blocks. These
produce types that satisfy the `Future` trait. Futures can be awaited using
`.await`:

```rust,ignore
async fn fetch_number() -> i32 {
    1337
}

async fn double_fetched() -> i32 {
    let n = fetch_number().await;

    n * 2
}
```

These `async` blocks get compiled to anonymous futures in the form of _finite
state machines_, which keep track of the future's progress, as well as their
scope contents.

## The `Future` trait

Implementing the `Future` trait allows using the `async/await` syntax on a type.
The trait is defined like this:

```rust,ignore
use std::pin::Pin;
use std::task::Context;

enum Poll<T> {
    Ready(T),
    Pending,
}

trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Futures are executed by calling the `poll` function, which advances them into
their next state.

When the coroutine executes and hasn't reached its final state yet, it returns
`Poll::Pending` and marks itself back to be polled via `cx` when it is ready.

When the coroutine is polled and finishes execution, it returns `Poll::Ready(T)`
with the result.

## Task waking

The `Context` type in `std::task` is used to carry context between coroutines,
as well as provide access to the `Waker` instance, which is used to let the
executor know that a future is ready to proceed.

Each time a future is polled, it is polled as a _task_, which is a top-level
future that is submitted to an _executor_.

The following example is a simple `Future` implementation that simply waits a
certain duration before resolving, and returns nothing:

```rust,ignore
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
```

The `poll` function simply checks whether the state is completed. If not, it
clones the waker to ensure it is located in the proper task, since futures are
passed around between tasks after being polled.

Constructing a new future:

```rust,ignore
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}
```

The future spawns a thread that simply waits a certain duration before marking
the future as complete, and waking the task using the `Waker` instance.

## Executors

An _executor_ takes _tasks_ and concurrently runs them to completion by calling
`poll` whenever they're ready to make progress.

### Task

To build a simple executor, a task has to be defined first. It is going to use
the `futures` crate which contains utilities for building executors:

```rust,ignore
use {
    futures::{
        future::BoxFuture,
        task::ArcWake,
    },
    std::{
        sync::mpsc::SyncSender,
        sync::{Arc, Mutex},
    },
};

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}
```

On the `Task` struct itself:

- `future` contains the underlying future to execute
- `task_sender` is a channel used by the task to send itself back into the queue

The `ArcWake` trait implementation allows waking up a specific task. It clones
the task itself and sends it to the sender.

### Spawner

Next, a `Spawner` is needed to create new tasks:

```rust,ignore
use std::sync::{Arc, Mutex, mpsc::SyncSender};

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}
```

The role of `Spawner` is submitting new tasks to the sender. The `spawn`
function takes a future, boxes it, assigns it to a task and sends for execution.

### Executor

Finally, `Executor` holds queue of tasks ready to be executed and handles the
event loop:

```rust,ignore
use {
    futures::task::waker_ref,
    std::{
        sync::mpsc::Receiver,
        sync::Arc,
        task::Context,
    },
};

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}
```

The `run` function accepts incoming tasks. The task's future is taken, and in
case it hasn't been completed yet, it is polled in an attempt to complete it.

If the future is still not complete after being polled, the future's returned to
the task, which will submit it to the `ready_queue` when it is able to proceed.

### Running

With everything set up, the spawner and executor can begin running tasks:

```rust,ignore
use std::{
    sync::mpsc::sync_channel,
    time::Duration,
};

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000; // just to make `sync_channel` happy
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("yolo");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("swag");
    });

    // Tells the executor that no more tasks will be coming
    drop(spawner);

    executor.run();
}
```

This prints `yolo`, and after two seconds prints `swag`. 🎉

There are many community runtimes available, such as `tokio`, that abstract
these low level details like implementing custom futures and building custom
executors.

## Pinning

By default, all types are _movable_. Primitive types like `i32` are passed
by-value, while fat pointers like `Box<T>` and `&mut T` allow swapping their
contents.

The type `Pin<P>` ensures that any _pointee_ of pointer `P` has stable location
in memory. This is essential for _self-referential types_, which many futures
are:

```rust,ignore
async {
    let mut x = [0; 128];
    let read_into_buf_fut = read_into_buf(&mut x);
    read_into_buf_fut.await;
    println!("{:?}", x);
}
```

This compiles down to something like this:

```rust,ignore
struct ReadIntoBuf<'a> {
    buf: &'a mut [u8], // points to `x` below
}

struct AsyncFuture {
    x: [u8; 128],
    read_into_buf_fut: ReadIntoBuf<'what_lifetime?>,
}
```

In case the future was moved, the `buf` pointer to `x` would suddenly have
pointed to an unknown location. To prevent this, `AsyncFuture` has to be
_pinned_ in order for `x` to stay in the same place.

For that reason, the `Future` trait's `poll` function takes a `Pin<&mut Self>`
as a receiver, ensuring that he future is pinned. This ensures it is not moved,
for example between threads.

### The `Unpin` trait

Primitive types are always freely movable because they do not require a stable
address in memory, such as `i32`, `bool` and references, as well as other types
composed of these types.

Types that do not need pinning implement the `Unpin` auto trait, which cancels
the effect of `Pin<P>`. For `T: Unpin`, `Pin<Box<T>>` is the same as `Box<T>`,
same for `Pin<&mut T>` and `&mut T`.

The `Unpin` trait only affects the _pointee_, not the _pointer_. In case of
`Pin<Box<T>>`, the `T` must be `Unpin`, and not `Box<T>`.

### The `!Unpin` marker

Self-referential types have to be marked as `!Unpin` using
`std::marker::PhantomPinned`, since they're not movable _without being pinned_:

```rust
use std::marker::PhantomPinned;

struct Test {
    text: String,
    ptr: *const String,
    _pin: PhantomPinned,
}

impl Test {
    fn new(text: &str) -> Self {
        let mut s = Self {
            text: text.to_string(),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        };
        s.ptr = &s.text;
        s
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn ptr(&self) -> &str {
        unsafe { &*(self.ptr) }
    }
}

fn main() {
    let mut test1 = Test::new("test1");
    let mut test2 = Test::new("test2");

    println!("{}, {}", test1.text(), test1.ptr()); // test1, test1
    std::mem::swap(&mut test1, &mut test2);
    println!("{}, {}", test2.text(), test2.ptr()); // test1, test2
}
```

Without pinning, swapping the memory of `test1` and `test2` causes `test2.ptr`
to suddenly point to a wrong location — the pointer _still_ points to the
`test1` struct, which already contains the `"test2"` string now.

For this type, operations like `std::mem::swap` are illegal, since they break
the pointer behavior. To prevent this, `Pin<T>` can be used to pin the `Test`
object into memory, so the `ptr` pointer will point to the correct location.

### Pinning to the stack

Pinning can be done on the _stack_ directly:

```rust
use std::marker::PhantomPinned;
use std::pin::Pin;

struct Test {
    text: String,
    ptr: *const String,
    _pin: PhantomPinned,
}

impl Test {
    fn new(text: &str) -> Self {
        let mut s = Self {
            text: text.to_string(),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        };
        s.ptr = &s.text;
        s
    }

    fn text(self: Pin<&Self>) -> &str {
        &self.get_ref().text
    }

    fn ptr(self: Pin<&Self>) -> &str {
        unsafe { &*(self.ptr) }
    }
}

fn main() {
    let mut test1 = Test::new("test1");
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };

    println!("{}, {}", test1.as_ref().text(), test1.as_ref().ptr());
    // std::mem::swap(test1.get_mut(), test2.get_mut()); // compilation error 🙀
    println!("{}, {}", test2.as_ref().text(), test2.as_ref().ptr());
}
```

The `std::mem::swap` function can no longer be used, because both `Test` objects
are now pinned and marked as `!Unpin`.

### Pinning to the heap

The `Box::pin` function can be used to pin objects to the heap:

```rust
use std::marker::PhantomPinned;
use std::pin::Pin;

struct Test {
    text: String,
    ptr: *const String,
    _pin: PhantomPinned,
}

impl Test {
    fn new(text: &str) -> Pin<Box<Self>> {
        let mut s = Self {
            text: text.to_string(),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(s);
        unsafe { boxed.as_mut().get_unchecked_mut().ptr = &boxed.as_ref().text as *const String };
        boxed
    }

    fn text(self: Pin<&Self>) -> &str {
        &self.get_ref().text
    }

    fn ptr(self: Pin<&Self>) -> &str {
        unsafe { &*(self.ptr) }
    }
}

fn main() {
    let mut test1 = Test::new("test1");
    let mut test2 = Test::new("test2");

    println!("{}, {}", test1.as_ref().text(), test1.as_ref().ptr());
    // std::mem::swap(test1.get_mut(), test2.get_mut()); // compilation error 🙀
    println!("{}, {}", test2.as_ref().text(), test2.as_ref().ptr());
}
```

The `ptr` field has to be populated _after_ the object is _boxed and pinned_, so
that it points to the correct location.

## Streams

The `Stream` trait is basically `Future` that yields multiple values:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Option<Self::Item>>;
}
```

The `poll_next` function returns `Poll::Pending` when the stream is waiting.
When values are ready, it returns `Poll::Ready(Some(T))`, and
`Poll::Ready(None)` when the stream is finished.

The `futures` crate has tools for both synchronous and asynchronous processing
of yielded values.
