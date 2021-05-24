# Modules

Modules control the organization, scope and privacy. They're defined by the
`mod` keyword:

```rust
mod blazer {
    // ...
}
```

## Crates

Crate is a _compilation unit_, an application entrypoint. They can be compiled:

- as a _binary_ using `rustc <root>.rs`
- as a _library_ using `rustc --crate-type=lib <root>.rs`

> It is recommended to use `cargo` for compiling configured in `Cargo.toml`
> instead of using `rustc` directly.

Absolute paths within a crate can be referenced using the `crate` keyword:

```rust
mod yolo {
    pub enum YoloSwag {
        Yolo,
        Swag,
    }
}

fn main() {
    use crate::yolo::YoloSwag;
    // ...
}
```

## Privacy

Everything is **private by default** and only made public by the `pub` keyword:

```rust
mod blazer {
    fn light_me_up() {
        // ...
    }

    pub fn blaze_it() {
        // ...
    }
}
```

This applies to all module's items, like _functions_, _traits_, _structs_,
_enums_, etc...

### Structs

Structs have per-field visibility which is private by default and made public
using `pub`:

```rust
mod auth {
    pub struct User {
        pub name: &str,
        // public
        created_at: u64,  // private
    }
}
```

Only _public_ structs can have public fields.

### Methods

Methods defined in `impl` blocks need to also be made public explicitly using
`pub`:

```rust
mod c {
    pub struct Count {
        count: u32,
    }

    impl Count {
        // public api
        pub fn inc(&mut self, times: u32) {
            for _ in 0..times {
                self.inc_by_one();
            }
        }

        // private implementation
        fn inc_by_one(&mut self) {
            self.count += 1;
        }
    }
}
```

### Scopes `self` and `super`

The `self` keyword is used for removing ambiguity, and accessing items
_relative_ to the current module, rather than using an _absolute_ path of the
current crate:

```rust
mod yolo {
    mod swag {
        pub enum YoloSwag {
            Yolo,
            Swag,
        }
    }

    fn go() {
        use self::swag::YoloSwag; // use crate::yolo::swag::YoloSwag;
        // ...
    }
}
```

The `super` keyword works as `self`, except it refers to the _parent module_:

```rust
mod yolo {
    mod swag {
        pub enum YoloSwag {
            Yolo,
            Swag,
        }
    }

    mod runner {
        fn go() {
            use super::swag::YoloSwag; // use crate::yolo::swag::YoloSwag;
            // ...
        }
    }
}
```

### Scope visibility

The `pub` keyword's scope can be specified explicitly:

- `pub(in path)` makes an item visible in the specified _path_ which must be an
  ancestor of the target item's module
- `pub(crate)` makes an item visible to the _crate_
- `pub(super)` makes an item visible to the _parent module_, same
  as `pub(in super)`
- `pub(self)` makes an item visible to the _current module_, same as
  `pub(in self)` or not using `pub` at all, thus being _private_

The _path_ specifier in `pub(in path)` must start with one of `crate`, `self` or
`super`.

## Use

The `use` keyword binds a full path to a new name, for easier access:

```rust
use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("file.txt")?;
    let m = HashMap::new();
    // ...
}
```

Multiple items can be specified within a path using `{}`:

```rust
use std::io::{self, Result};

fn main() -> Result<()> {
    let e = io::empty();
    // ...
    Ok(())
}
```

An _alias_ can be specified for an item using `as`:

```rust
use std::fs as stdfs;

fn main() {
    let file = stdfs::read_to_string("file.txt")?;
    // ...
}
```

Paths are _relative_ by default. An _absolute path_ can be specified for
disambiguation:

```rust
mod std {
    pub fn fs() {
        // ...
    }
}

fn main() {
    use ::std::fs as stdfs;  // absolute external crate path
    use self::std::fs;       // explicit relative path
}
```

### Re-exporting

Names can be _re-exported_ using `pub use` to create flatter APIs:

```rust
mod yolo {
    mod swag {
        pub enum YoloSwag {
            Yolo,
            Swag,
        }
    }

    pub use swag::{YoloSwag};
}

fn main() {
    use yolo::{YoloSwag};  // instead of yolo::swag::{YoloSwag}
    // ...
}
```

A wildcard `*` can be used for using everything, although commonly only used
with re-exporting:

```rust
pub use lib::nested::*;
```

### Enum variants

Enum variants are sometimes used within a function for cleaner `match` syntax:

```rust
mod swag {
    pub enum YoloSwag {
        Yolo,
        Swag,
    }
}

fn main() {
    use swag::YoloSwag::*;

    let s = Yolo;
    match s {
        Yolo => println!("Yolo"),
        Swag => println!("Swag"),
    };
}
```

### Conventions

The `use` path binding conventions are:

- using whole modules for their public API, `use std::io;`
- using _structs_, _enums_ and _traits_ directly,
  `use std::collections::HashMap;`
- sometimes using _enum variants_ with wildcards in
  functions, `use lib::Enum::*;`
- re-exporting to create flatter APIs, `pub use lib::nested::*;`

## File hierarchy

_Files_ and _folders_ can act as modules. The `mod <name>;` declaration can be
used to bring `<name>.rs` or `<name>/mod.rs` items under the current scope
inside a module named `<name>`:

```rust
// <root>/custom.rs or <root>/custom/mod.rs
pub fn do_stuff() {
    // ...
}

// some file in <root>
mod custom;

fn stuff() {
    custom::do_stuff();
}
```

The `pub mod <name>;` declaration can be used for re-exporting:

```rust
// <root>/custom/core.rs
pub fn do_stuff() {
    // ...
}

// <root>/custom/mod.rs
pub mod core;

// some file in <root>
mod custom;

fn stuff() {
    custom::core::do_stuff();
}
```
