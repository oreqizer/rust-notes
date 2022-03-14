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

## `Send` and `Sync` traits

The only two language features for concurrency — all others are just an
abstraction above these.

The `Send` trait indicates that the type's ownership can be _transferred between
threads_. Almost all types implement this automatically, except `Rc<T>`, raw
pointers, and few others.

The `Sync` trait indicates that it is safe for the type implementing `Sync` to
be referenced from multiple threads. Most types are `Sync`, except runtime
borrow types like `RefCell<T>`.

> Any type `T` is `Sync` if `&T` is `Send`.

All compound types that are composed of types that implement `Send` also
implicitly implement `Send`. Same goes for `Sync`.

## Arc

Thread safe version of `Rc<T>`, the `Arc<T>`, or _atomic reference counter_,
implements the `Clone` trait that creates a reference pointer to a value on the
heap:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let swag = Arc::new("swag");

    for _ in 0..10 {
        let swag = swag.clone();

        thread::spawn(move || {
            println!("{:?}", swag); // swag is moved
        });
    }
}
```

It behaves just like `Rc<T>`, but has a slight performance cost due to the
thread safety.

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

## Mutex and RwLock

Short for _mutual exclusion_, it is a mechanism for ensuring only a single
thread accesses a mutable memory at one time.

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // MutexGuard goes out of scope, mutex is released

    println!("m = {:?}", m);
}
```

The type `T` in a `Mutex<T>` must be `Send`.

Sharing a `Mutex<T>` instance between multiple threads is achieved using
`Arc<T>` that handles reference counting:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    (0..10)
        .map(|_| {
            let counter = Arc::clone(&counter);

            thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            })
        })
        .for_each(|h| h.join().unwrap());

    println!("Result: {}", *counter.lock().unwrap());
}
```

### RwLock

Basically `Mutex<T>`, except it allows _multiple readers_ and _one writer_:

```rust
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    let counter = Arc::new(RwLock::new(0));

    let pc = counter.clone();
    let producer = thread::spawn(move || loop {
        if let Ok(mut w) = pc.write() {
            *w += 1;
            thread::sleep(Duration::from_millis(500));
        }
    });

    let rc_1 = counter.clone();
    let consumer_1 = thread::spawn(move || loop {
        if let Ok(v) = rc_1.read() {
            // One reads here
            println!("counter   = {}", v);
            thread::sleep(Duration::from_millis(250));
        }
    });

    let rc_2 = counter.clone();
    let consumer_2 = thread::spawn(move || loop {
        if let Ok(v) = rc_2.read() {
            // Another one reads here, Mutex would deadlock
            println!("counter^2 = {}", *v * *v);
            thread::sleep(Duration::from_millis(250));
        }
    });

    producer.join().unwrap();
    consumer_1.join().unwrap();
    consumer_2.join().unwrap();
}
```

If this was done with a `Mutex<T>`, the readers would get deadlocked. The
drawback is that in `RwSync<T>`, the `T` must be both `Send` and `Sync`.
