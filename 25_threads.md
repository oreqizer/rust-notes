# Threads

Rust has a _1:1 thread system_ — every thread is a physical thread managed by
the OS. Thread is spawned using `thread::spawn`:

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

Channels send data from multiple threads into a single consuming thread to
process the results:

```rust
use std::sync::mpsc; // multiple producers single consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("swag").unwrap();
    });

    let r = rx.recv().unwrap();
    println!("yolo {}", r);
}
```

The `rx` receiver can also be used as an _iterator_.

The _transmitter_ `tx` needs to be cloned to be used in multiple threads:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    (1..10)
        .map(|x| {
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(x).unwrap();
            })
        })
        .for_each(|h| h.join().unwrap());

    drop(tx); // rx waits until all clones of tx are dropped
    for r in rx {
        println!("{}", r);
    }
}
```

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
