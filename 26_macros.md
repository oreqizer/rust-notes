# Macros

Rust's way of metaprogramming. Macros allow _compile-time_ code generation that
generates source code based on supplied parameters.

## Declarative macros

Macros that utilize
[special DSL](https://doc.rust-lang.org/reference/macros-by-example.html) to
write source code in a concise and readable way.

> Current `macro_rules!` will be deprecated and replaced with
> [declarative macros 2.0](https://github.com/rust-lang/rust/issues/39412)

Declarative macros are invoked like functions ending with `!`, like `println!`.
They are created using the `macro_rules!` keyword followed by the macro name.

Arguments passed to macros are matched against the pattern specified by the DSL,
consisting of _designators_ bound to names prefixed with `$`:

```rust
#[macro_export]
macro_rules! create_function {
    ($name:ident) => {
        fn $name() {
            println!("You called {:?}", stringify!($name));
        }
    };
}

create_function!(kek); // expands into:
// fn kek() {
//   println!("You called {:?}", "kek");
// }

fn main() {
    kek(); // You called "kek"
}
```

Macros can optionally be exported by the `#[macro_export]` macro. They're then
brought into scope when the crate they're in is brought into scope.

### Designators

There's a bunch of available designators to match against:

- `block`
- `expr` is used for expressions
- `ident` is used for variable/function names
- `item`
- `literal` is used for literal constants
- `pat` _(pattern)_
- `path`
- `stmt` _(statement)_
- `tt` _(token tree)_
- `ty` _(type)_
- `vis` _(visibility qualifier)_

This macro prints the expression and its result:

```rust
macro_rules! print_result {
    ($e:expr) => {
        println!("{:?} = {:?}", stringify!($e), $e);
    };
}

fn main() {
    print_result(2 * 2); // "2 * 2" = 4
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });                  // "{ let x = 1u32; x * x + 2 * x - 1 }" = 2
}
```

### Repetitions

Arbitrary number of designators, including optional ones, can be _matched_ and
_expanded_ using:

- `*` for 0 to infinity
- `+` for 1 to infinity
- `?` for 0 or 1

A naïve implementation of `vec!` without memory optimization would look like
this:

```rust
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )* // repeats as many times as $x
            temp_vec
        }
    };
}

fn main() {
    let _v = vec!(1, 2, 3);
}
```

When expanding, the same kind and nesting must be used than when matching. When
expanding multiple matchers in one repetition, their number of fragments must be
the same.

### Overload

_TODO_

## Procedural macros

_TODO_
