# Threads

Rust has a _1:1 thread system_ — every thread is a physical thread managed by the
OS. Thread is spawned using `thread::spawn`:

```rust
use std::thread;

fn main() {
    for i in 1..10 {
        // may or may not finish
        thread::spawn(move || {
            println!("i = {}", i);
        });
    }
}
```

To wait for a thread from the parent thread, the spawned thread's _handle_ can
be _joined_ using the `join` method:

```rust
use std::thread;

fn main() {
    (1..10)
        .map(|i| thread::spawn(move || println!("i = {}", i)))
        .for_each(|h| h.join().unwrap()) // all will finish
}
```

## Channels

_TODO_

## The `Sync` trait

_TODO_

## Arc

Thread safe version of `Rc<T>`, the `Arc<T>`, or _atomic reference counter_,
implements the `Clone` trait creates a reference poitner to a value on the heap:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let swag = Arc::new("swag");

    for _ in 0..10 {
        let swag = Arc::clone(&swag);

        thread::spawn(move || {
            println!("{:?}", swag); // swag is moved
        });
    }
}
```

## RWLock

_TODO_

## Mutex

_TODO_
