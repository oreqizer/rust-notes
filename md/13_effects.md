# Effects

Constructs for handling errors and optional values.

## Option

The `Option<T>` enum is for optional values:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

It serves as a replacement for `nil` or empty values:

```rust
fn only_big_nums(x: i32) -> Option<i32> {
    if x > 1000 { Some(x) } else { None }
}

fn main() {
    let n = only_big_nums(1337);

    match n {
        Some(_) => println!("That is big!"),
        None => println!("Small"),
    }
}
```

## Result

The `Result<T, E>` enum is for operations that can fail and contains either a _
value_ or an _error_:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The main use is for operations that can fail with a recoverable error:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

## Operator `?`

The `?` operator can be used with both `Option` and `Result` when calling
functions that return these types:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}
```

The `?` operator placed after a function that returns an `Option` or `Result`
_unwraps_ the value if the result is `Some(T)` or `Ok(T)`. If the funciton ends
with `None` or `Err(E)`, it is returned from with this value.

## Panic

The `panic!` macro is used for _unrecoverable errors_. It and terminates the
current thread with an error message:

```rust
fn main() {
    let f = File::open("dataset.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => panic!("could not open dataset"),  // boom 💥
    };
    // ...
}
```

A `panic!` called in the main thread terminates all other threads and exits the
program with an error code `101`.

### Unwrap

Calling the `.unwrap()` method on `Option` or `Result` values unwraps the
underlying value if it is `Some(T)` or `Ok(T)`, and calls `panic!` otherwise:

```rust
fn main() {
    let mut f = File::open("dataset.txt").unwrap();  // boom 💥 or file 🗂
    // ...
}
```

Variants:

- `.unwrap_or(T)` returns the supplied default if `None` or `Err(E)`
- `.unwrap_or_default()` returns the type's default value if `None` or `Err(E)`
- `.unwrap_or_else(Fn(E) -> T)` calls the closure if `None` or `Err(E)`
- `.unwrap_err()` panics with the value if value is `Some(T)` or `Ok(T)`

Useful when the programmer knows more than the compiler, and is sure that the
code at hand will never fail.

### Expect

The same as `.unwrap()`, but with a meaningful error message:

```rust
fn main() {
    let mut f = File::open("dataset.txt").expect("oops");  // boom 💥 or file 🗂
    // ...
}
```

Variants:

- `.expect_err()` panics with the value if value is `Some(T)` or `Ok(T)`
