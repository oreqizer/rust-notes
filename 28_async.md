# Async

Asynchronous programming is a _concurrent programming model_ allowing executing
many concurrent tasks on a small number of OS threads.

Rust's implementation of async is:
- **zero-cost** - can be performed without heap allocations and dynamic dispatch
- **lazy** - progress is made only when polled
- **zero-runtime** - runtime implementation is provided by community crates
- **single- and multi-threaded** - depending on implementation

## The `Future` trait

_TODO_

## Pinning

_TODO_

## Streams

_TODO_
