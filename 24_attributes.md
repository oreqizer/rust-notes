# Attributes

An attribute is metadata applied to some module, crate or item. Syntax:

- crate-wide attributes `#![crate_attribute]`
- module and item attributes `#[item_attribute]`

Attributes can take arguments:

- `#[attr = "value"]`
- `#[attr(key = "value")]`
- `#[attr(value)]`

## Disabling lints

Attribute `#[allow]` can be used for ignoring compiler warnings when linting:

```rust
#[allow(dead_code)]
fn yo() {
    println!("do I even exist?");
}

fn main() {
    println!("totally not calling 'yo'");
}
```

## Crates

Some common crate attributes include:

- `#![crate_type]` for differentiating binaries and libraries, has no effect
  when using `cargo` and can be specified via `rustc`'s `--crate-type` flag
- `#![crate_name]` for naming the crate, has no effect when using `cargo` and
  can be specified via `rustc`'s `--crate-name` flag
- `#![feature(...)]` for enabling experimental compiler features in **Nightly
  Rust**

## Cfg

The `#[cfg(...)]` attribute and `cfg!(...)` macro can be used for conditional
compilation and runtime compiler checks:

```rust
#[cfg(target_os = "linux")]
fn on_linux() {
    // compiled on Linux
    println!("Nerd");
}

#[cfg(not(target_os = "linux"))]
fn not_on_linux() {
    // compiled on everything but Linux
    println!("Still a nerd");
}

fn main() {
    // checked at runtime
    if cfg!(target_os = "linux") {
        println!("Nerd");
    } else {
        println!("Still a nerd");
    }
}
```

This pattern is useful for building libraries with different feature set based
on the compilation environment or feature flagging.

### Features

The convention of `cargo` for feature flagging is using the `feature` config
attribute:

```rust
#[cfg(feature = "yolo")]
pub mod yolo {
  // ...
}

#[cfg(feature = "swag")]
pub mod swag {
  // ... uses 'yolo' internally
}
```

The `Cargo.toml` file then specifies the list of possible features with their
feature dependencies:

```toml
[features]
yolo = [] # no feature dependencies
swag = ["yolo"] # also enables 'yolo'
```

The optional `default` feature can specify features that are enabled by default:

```toml
[features]
default = ["yolo"]
yolo = []
swag = ["yolo"]
```

The default featue set can be disabled using `--no-default-features`. See
[Cargo feature docs](https://doc.rust-lang.org/cargo/reference/features.html)
for more details and how to consume feature flags in dependencies.

### Custom

Some conditions like `target_os` are built into the compiler. Custom ones can
be specified when compiling with the `--cfg` flag:

```rust
#[cfg(yolo)]
fn yolo() {
    println!("swag");
}

fn main() {
    yolo();
}
```

This code will only work when compiling using `rustc --cfg yolo`.
