# Tests

Tests can be run by `rustc --test` or `cargo test`, which runs functions marked
with the `#[test]` attribute.

## Unit

The convention is writing _unit tests_ right next to the production code:

```rust
fn do_stuff() {
    // ...
}

#[test]
fn test_do_stuff() {
    do_stuff();
}
```

It is common to declare a module marked with `#[cfg(test)]` to co-locate tests
and testing utilities:

```rust
fn do_stuff() {
    // ...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_stuff() {
        do_stuff();
    }

    #[test]
    fn test_do_stuff_twice() {
        do_stuff();
    }
}
```

## Integration

Integration tests live in the `tests` folder next to the `src` folder. They're
meant to test the crate's public API:

```rust
// Crate 'yolo':
// src/lib.rs
fn swag() {
    // ...
}

// tests/lib.rs
use yolo;

#[test]
fn test_swag() {
    yolo::swag();
}
```

No need to use the `#[cfg(test)]` attribute in the `tests` folder, as everything
is a test.

## Assertions

The `assert!` macro asserts a boolean expression. All assertions can optionally
have a custom message:

```rust
#[test]
fn test_stuff() {
    assert!(2 + 2 == 4); // uses the default assertion message
    assert!(2 + 2 == 4, "two plus two better fucking equal four");
}
```

The `assert_eq!` and `assert_ne!` macros are used for comparing equality:

```rust
#[test]
fn test_stuff() {
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
}
```

By default, if a test panics, it is considered failed. The `#[should_panic]`
attribute is useful when a test _should_ panic:

```rust
fn do_wild_things() {
    panic!("Yoooooo!");
}

#[test]
#[should_panic]
fn test_wild_stuff() {
    do_wild_things();
}
```

To avoid accidentally passing an unexpected `panic!` call, an expected substring
of the panic message can be specified:

```rust
fn do_wild_things() {
    panic!("Yoooooo!");
}

#[test]
#[should_panic(expected = "Yooo")]
fn test_wild_stuff() {
    do_wild_things();
}
```

Tests can also return `Result<T, E>` instead of panicking, which allows using
the `?` operator:

```rust
fn blaze_it() -> Result<i32, String> {
    // ...
    Ok(420)
}

#[test]
fn test_blaze_it() -> Result<(), String> {
    let n = blaze_it()?;
    match n {
        420 => Ok(()),
        _ => Err(String::from("wrong 🙀")),
    }
}
```

## Docs

The `cargo doc` command generates documentation, optionally with the `--open`
flag, based on special comments written in **Markdown**.

Comments starting as `//!` are used for documenting _crates_ as a whole, or
individual _modules_. They're put at the top of a file:

```rust
//! # Calcx
//!
//! `calcx` is a collection of utilities to make performing certain
//! calculations more convenient.
```

Comments in the form `///` document individual items in a module:

```rust
/// Adds one to the number given.
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Commonly seen sections:

- `# Examples` minimal examples that show the usage of the item
- `# Panics` describes scenarios when a function calls `panic!`
- `# Errors` when returning `Result<T, E>`, this section describes cases when
  `Err` is returned
- `# Safety` if the function uses `unsafe` code, specifies invariants that the
  function expects callers to uphold

### Doc tests

When specifying code blocks in `///` comment docs for libraries, `cargo test`
runs these tests, and they have their own section called _doc-tests_:

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = calcx::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Running this will produce:

```
   Doc-tests calcx

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s
```
