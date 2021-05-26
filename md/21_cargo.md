# Cargo

The official package manager for Rust. The `cargo` CLI contains various tools
for working with Rust and its bundled tools like `rustc` and `rustfmt`.

Overview of the most common commands:

- `cargo new <name>` creates a new _binary_ project
- `cargo new --lib <name>` creates a new _library_ project
- `cargo install <crate>` installs a dependency from [Crates.io](https://crates.io)
- `cargo fmt` formats source code
- `cargo fix` fixes source code based on compiler hints
- `cargo test` runs tests
- `cargo run` runs a binary, `src/main.rs` by default
- `cargo build` builds the development version
- `cargo build --release` builds the production version
- `cargo doc` generates documentation
- `cargo publish` publishes a library

The `Cargo.toml` file contains `cargo` configuration, like the crate name,
dependencies, language version.

## New project

For a binary, run `cargo new <name>`, which generates:

```
src/
  main.rs
Cargo.lock
Cargo.toml
```

For a library, run `cargo new --lib <name>`, which generates:

```
src/
  lib.rs
Cargo.lock
Cargo.toml
```

Can also have CLI in `src/main.rs`.

## Dependencies

The `Cargo.toml` file specifies the project's _version_ and _dependencies_. The
common convention is following _SemVer_ for versioning.

Specifying a verion number as a crate version downloads the library
from [Crates.io](https://crates.io). Fetching a specific _Git_ repository or a _
file path_:

```toml
[package]
name = "yolo"
version = "4.2.0"
authors = ["oreqizer"]

[dependencies]
clap = "^2.27.1" # from crates.io
slap = { version = "^1.3.37", registry = "kratos.io" } # from custom registry
rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
swag = { path = "../swag" } # from a path in the local filesystem
```

A library project can be published to [Crates.io](https://crates.io) by running
`cargo publish`.

## Binaries

In addition to the main `src/main.rs` binary, other binaries can be specified in
the `bin` folder:

```
src/
  bin/
    stuff1.rs
    stuff2.rs
  ...
Cargo.lock
Cargo.toml
```

In order to work with these binaries instad of `src/main.rs`, the `--bin` flag
can be specified like `cargo run --bin stuff1`

## Build script

The `build.rs` can be created to run a script before the build begins, such as
code generation or native code.

The file name can be adjusted in `Cargo.toml`:

```toml
[package]
build = "build.rs"
```

## Workspaces

Growing projects can be assembled in the form of _workspaces_. Specifying
workspaces is done by creating a root `Cargo.toml` file:

```toml
[workspace]
members = [
    # ...
]
```

In this folder, create members by `cargo run` and add their names to
the `members` array in root `Cargo.toml`. The structure looks something like:

```
app/
  src/
    main.rs
  Cargo.lock
  Cargo.toml
utils/
  src/
    lib.rs
  Cargo.lock
  Cargo.toml
Cargo.toml
```

The root `Cargo.toml`:

```toml
[workspace]
members = [
    "app",
    "utils",
]
```

To import `utils` into `app`, specify a relative path
in `app/Cargo.toml`:

```toml
[depencencies]
utils = { path = "../utils" }
```

Commands are meant to be run directly in the _root of the project_, such as
`cargo test` or `cargo build`.

Some commands like `cargo run` require the `-p` flag that specifies which
project to run the command on, like `cargo run -p app`.
