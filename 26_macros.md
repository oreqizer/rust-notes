# Macros

Rust's way of metaprogramming. Macros allow _compile-time_ code generation that
generates source code based on supplied parameters.

See [tlborm](https://veykril.github.io/tlborm/) for details on macros!

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

### Matching

Macro pattern matching is done against lexical tokens and allows _multiple
signatures_ and _recursion_:

```rust
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}
```

Arbitrary tokens can be matched:

```rust
macro_rules! assert_many {
    ($($next:expr),*; one $msg:tt) => {
        assert!(false $(|| $next)*, stringify!($msg));
    };
    ($($next:expr),*; all $msg:tt) => {
        assert!(true $(&& $next)*, stringify!($msg));
    };
}

fn main() {
    assert_many!(2 == 2, 5 > 8; one "one is enough");
    assert_many!(2 == 2, 5 < 8; all "all must pass");
}
```

## Procedural macros

_Procedural macros_ are functions that take `TokenStream` input and output a
modified `TokenStream`. There are three kinds:

- Function-like macros - `custom!(...)`
- Derive macros - `#[derive(Custom)]`
- Attribute macros - `#[custom]`

Procedural macros have to be defined in a separate crate with the following
field in `Cargo.toml`:

```toml
[lib]
proc-macro = true
```

### Function-like macros

These macros are invoked with the `custom!()` syntax. They're defined using the
`#[proc_macro]` attribute:

```rust
// custom_macros crate
use proc_macro::TokenStream;

#[proc_macro]
pub fn custom(_input: TokenStream) -> TokenStream {
    // normally you'd modify input here and return the result
    r#"fn yolo() -> &'static str { "swag" }"#.parse().unwrap()
}
```

You then include it as a dependency in your regular crate and use it:

```rust
// your regular crate
use custom_macros::custom;

custom!();

fn main() {
    println!("yolo {}", yolo()); // yolo swag
}
```

### Derive macros

_Derive macros_ define inputs for the `#[derive(...)]` attribute. They can be
applied on `struct`, `enum` or `union` token streams:

```rust
// custom_macros crate
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Blaze)]
pub fn blaze_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl Blaze for #name {
            fn it() {
                println!("🔥");
            }
        }
    };
    gen.into()
}
```

The macro only needs the trait's _name_, it does not need to actually be in
scope when defining the macro function.

Bringing the derive trait's name into scope applies the macro:

```rust
// your regular crate
use custom_macros::Blaze;

pub trait Blaze {
    fn it();
}

#[derive(Blaze)]
struct Snoop;

fn main() {
    Snoop::it(); // 🔥
}
```

Derive macros can define _helper attributes_:

```rust
// custom_macros crate
use proc_macro::TokenStream;

#[proc_macro_derive(Custom, attributes(custom))]
pub fn custom_derive_attr(input: TokenStream) -> TokenStream {
    println!("input = {}", input.to_string());
    TokenStream::new()
}
```

These are _inert_ and can be specified on items as additional information:

```rust
// your regular crate
use custom_macros::Custom;

pub trait Custom {
    fn it();
}

// prints during compilation:
// input = struct Struct { #[custom] field : u32 }
#[derive(Custom)]
struct Struct {
    #[custom] field: u32
}
```

### Attribute macros

_Attribute macros_ define new outer attributes using the
`#[proc_macro_attribute]` attribute. They take the _attribute token stream_ and
the _item token stream_ as arguments:

```rust
// custom_macros crate
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn print_tokens(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: {}", attr.to_string());
    println!("item: {}", item.to_string());
    item
}
```

Attribute macros can take arbitrary arguments to adjust their behavior:

```rust
// custom_macros regular crate
use custom_macros::print_tokens;

// prints during compilation:
// attr: GET, "/:id"
// item: fn get_username(id : u8) -> String { "xxx_bobby_xxx".to_string() }
#[print_tokens(GET, "/:id")]
fn get_username(_id: u8) -> String {
    "xxx_bobby_xxx".to_string()
}
```
