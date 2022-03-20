# Unsafe

The `unsafe` keyword turns off memory safety checks by the compiler. It is used
to mark code that can potentially be unsafe, such as:

- dereferencing raw pointers
- calling other unsafe functions
- implementing unsafe traits
- accessing or modifying a static mutable variable
- foreign function interfacing

## Raw pointers

Immutable raw pointers are created by `*const T`, mutable ones by `*mut T`.
Creating raw pointers is considered _safe_, only their dereferencing is not:

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;   // immutable "borrow"
    let r2 = &mut num as *mut i32; // can have mutable "borrow" 🙀

    unsafe {
        println!("r1 = {}", *r1); // r1 = 5
        println!("r2 = {}", *r2); // r2 = 5
    }
}
```

Arbitrary address can be pointed to:

```rust
fn main() {
    let shithole = 0x12345 as *const i32;
    // println!("what is here? {}", *shithole); // please don't 🙀
}
```

 ## Unsafe functions

 _TODO_ calling unsafe fns, creating safe abstractions