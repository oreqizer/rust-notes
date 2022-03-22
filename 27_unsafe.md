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

Functions can be marked unsafe using the `unsafe fn` declaration:

```rust
unsafe fn do_danger() {
    // ...
}
```

Calling unsafe functions is considered unsafe. To use these functions in a safe
environment, a safe abstraction can be created:

```rust
use std::slice;

fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // slice::from_raw_parts_mut is an unsafe function
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut s = "420blazeit".chars().collect::<Vec<char>>();
    let (a, b) = split_at_mut(&mut s, 3);

    println!("a = {:?}", a); // a = ['4', '2', '0']
    println!("b = {:?}", b); // b = ['b', 'l', 'a', 'z', 'e', 'i', 't']
}
```

## Unsafe traits

Implementing an `unsafe` trait is considered unsafe:

```rust
unsafe trait Yolo {
    //
}

unsafe impl Yolo for i32 {
    //
}
```

An example of an unsafe trait is creating a trait that holds a type that is not
`Send` or `Sync` such as a raw pointer, and we want to mark the trait as `Send`
or `Sync`.

## Mutable static variables

Accessing and mutating _mutable static variables_ is unsafe:

```rust
static mut GLOBAL_ID: i32 = 0;

fn main() {
    unsafe {
        GLOBAL_ID += 1;
        println!("id = {}", GLOBAL_ID);
    }
}
```

## FFI

Calling a _foreign function interface_ functions defined in `extern`
declarations is unsafe:

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("abs(-420) according to C is {}", abs(-420));
    }
}
```
