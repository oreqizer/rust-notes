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

The `Result<T, E>` enum is for operations that can fail and contains
either a _value_ or an _error_:

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
_unwraps_ the value if the result is `Some(T)` or `Ok(T)`. If the funciton
ends with `None` or `Err(E)`, it is returned from with this value.

## Panic

The `panic!` macro is used for _unrecoverable errors_. It prints an error
message on the screen, the stack trace and terminates the current thread:

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

A `panic!` called in the main thread exits the program with an error code.
