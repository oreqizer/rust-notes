# Intro

**Rust** is a compiled, statically and strongly typed language. Main features:
- memory safety
- low-level performance
- zero-cost abstractions
- non-nullable references

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

## Conventions

* `snake_case` for _functions_, _variables_, _macros_ and _lifetimes_
* `PascalCase` for _enums_, _structs_, _generics_ and _traits_
* `SCREAMING_SNAKE_CASE` for _constants_ and _statics_
