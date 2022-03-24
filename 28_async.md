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

The `async/await` syntax gets compiled to a finite state machine that executes
when the `poll` function is called.

When the coroutine executes and needs to wait before proceeding, it returns
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

## Pinning

_TODO_

## Streams

_TODO_
