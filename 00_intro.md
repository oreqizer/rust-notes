# Intro

**Rust** is a compiled, statically and strongly typed language. Main features:
- memory safety
- low-level performance
- zero-cost abstractions
- non-nullable references

## Language

Overview of Rust's main constructs and conventions.

### Constructs

* **primitives** like `i32`, `f64`
* fixed-size **arrays** have values of the same type, `[i32; 100]`
* **tuples** are collections of values of different types, `(i32, i32, &str)`
* **structs** hold related data and define related **methods**, `vector.normalize()`
* **references** allow _borrowing_ data, `&value`
* **lifetimes** ensure data validity, `'a`
* **traits** define behavior that types have like `ToString`
* **closures** are anonymous functions that capture their context, `|x| x * 2`
* **generics** allow parameters of different types `Vec<T>`
* declarative and procedural **macros** for metaprogramming, `println!("a macro")`
  or `#[derive(Debug)]`

### Comments

Comments are specified after `//`. No special multiline syntax.

### Conventions

* `snake_case` for _functions_, _variables_, _macros_ and _lifetimes_
* `PascalCase` for _enums_, _structs_, _generics_ and _traits_
* `SCREAMING_SNAKE_CASE` for _constants_ and _statics_

## Setup

* install `rustup`
* use `cargo` for development

### Binary

`cargo new <name>`

Generates
```
src/
  main.rs
Cargo.lock
Cargo.toml
```

### Library

`cargo new --lib <name>`

Generates
```
src/
  lib.rs
Cargo.lock
Cargo.toml
```

Can also have CLI in `src/main.rs`.

### Scripts

* `cargo test` runs tests
* `cargo fmt` formats source code
* `cargo run` runs `src/main.rs`
* `cargo build` builds the development version
* `cargo build --release` builds the production version
* `cargo doc` generates documentation
* `cargo publish` publishes a library
