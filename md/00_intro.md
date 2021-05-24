# Intro

**Rust** is a compiled, statically and strongly typed language. Main features:

- memory safety
- low-level performance
- zero-cost abstractions
- non-nullable references

Install Rust via `rustup`, which installs all Rust tools and `cargo` — Rust's
official package manager and orchestration tool.

For a playground during learning, run `cargo new playground`, edit `main.rs` and
run by `cargo run` in the project root directory.

## Constructs

* **primitives** like `i32`, `f64`, `bool`
* **arrays** are fixed-size collections of values of the same type, `[i32; 100]`
* **tuples** are collections of values of different types, `(i32, i32, &str)`
* **references** allow _borrowing_ data, `&value`
* **slices** offer referencing data sequences with a variable size, `[T]`, `str`
* **structs** hold related data and define related **methods**
  , `vector.normalize()`
* **enums** define _variants_ of types, `Result<T, E>` and its `Ok(T)`, `Err(E)`
* **generics** allow parameters of different types `Vec<T>`
* **traits** define behavior that types have like `ToString`
* **DSTs** are types with size not known at compile-time, `dyn Trait`, `[T]`
  , `str`
* **lifetimes** ensure data validity, `'static`,`'a`
* **closures** are anonymous functions that capture their context, `|x| x * 2`
* **macros** for metaprogramming, declarative and
  procedural, `println!("a macro")`
  or `#[derive(Debug)]`

## Comments

Comments are specified after `//`. No special multiline syntax.

## Conventions

* `snake_case` for _crates_, but prefer single-word
* `snake_case` for _functions_, _variables_, _macros_ and _lifetimes_
* `PascalCase` for _enums_, _structs_, _generics_ and _traits_
* `SCREAMING_SNAKE_CASE` for _constants_ and _statics_
