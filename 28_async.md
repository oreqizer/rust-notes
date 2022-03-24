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

```rust
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

```rust
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

```rust
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

```rust
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

_TODO_

## Pinning

_TODO_

## Streams

_TODO_
