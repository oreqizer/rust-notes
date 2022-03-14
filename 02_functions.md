# Functions

Declared using `fn` and arguments, return type by `->`.

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}
```

Functions can be used before they are declared:

```rust
fn add_two(x: i32) -> i32 {
    add_one(add_one(x))
}

fn add_one(x: i32) -> i32 {
    x + 1
}
```

## Return

Like in regular blocks, the last _expression_ in a function block is the _return
value_:

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

// is the same as
fn add_one_return(x: i32) -> i32 {
    return x + 1;
}
```

## Main

Function `main` in `src/main.rs` is the application entrypoint.

It returns either `()`:

```rust
// src/main.rs
fn main() {
    println!("kek");
}
```

Or `Result<(), E>`:

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
```

## Pointers

Functions can be passed around as _pointers_:

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn add_two(x: i32) -> i32 {
    do_twice(add_one, x)
}
```

## Const functions

Functions that only operate with arguments and operations performable at
_compile-time_ can be marked as `const fn` and used in such contexts:

```rust
const X: i32 = 1337;
const X_TWO: i32 = double(X); // 2674

const fn double(x: i32) -> i32 {
    x * 2
}
```
