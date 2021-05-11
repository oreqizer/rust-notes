# Variables

Every variable and reference is **immutable** by default.

## Declaration

* `let` declares a variable
* `let mut` declares a _mutable_ variable
* `const` declares a constant that gets inlined during compilation
* `static` declares a variable with a static location in memory
* `static mut` declares a _mutable_ variable with a static location in memory

## Variables

Variables declared using `let` or `let mut`:

```rust
fn main() {
    let x: i32 = 5;

    let mut y: i32 = 10;
    y = 12;
}
```

Type can be inferred:

```rust
fn main() {
    let x = -4; // i32
}
```

Initialization can be deferred, _type annotation_ is optional:

```rust
fn main() {
    let mut z;
    // ...
    z = 6;
}
```

Initializing the same variable multiple times _shadows_ the previous
declaration, types can differ:

```rust
fn main() {
    let x = 5;
    // ...
    let x = "kek";
}
```

## Statics

Static variables are declared using `static` in the global scope. They can be
_mutable_, although mutable static variables is a bad practice because they're
not thread safe. They must have a _type annotation_.

Static variables have a static place in memory and can be passed around as
references.

```rust
static NICKNAME: &str = "henchbruv";
static mut PLAYERS: u32 = 0; // :(
```

## Constants

Constants are declared using `const` and are values that are inlined during
compilation. They can be declared in any scope. They must have a _type
annotation_.

```rust
const MAX_VALUE: u32 = 1337;
```

Unless interior mutability or a static place in memory is required, constants
are preferred over statics.
