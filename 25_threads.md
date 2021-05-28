# Threads

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
