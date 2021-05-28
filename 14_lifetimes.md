# Lifetimes

Lifetimes ensure that _references_ point to a valid memory. Lifetime of a
reference is as big as is the _scope_ of a variable:

```rust
fn main() {
    let x = 13;       // --------+- 'a
    let xr = &x;      //         |
    {                 //         |
        let y = 37;   // --+- 'b |
        let yr = &y;  //   |     |
    }                 // --+     |
}                     // --------+
```

_Lifetime_ denotes scope, _lifetime-parameter_ denotes parameters that the
compiler substitutes with a real lifetime, same as inferring types of generics.

Lifetime-parameters are named `'a`, an apostrophe and a lowercase name, often
just a single letter, notable exception being `'static`. As generics, they're
declared using the `<>` syntax:

```rust
fn gimme_bigger<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

In this case, references `s1` and `s2` may have different lifetimes, and it is
impossible to determine the returned reference's lifetime at compile-time.

When a lifetime-parameter is used on multiple references, the compiler choses _
the shortest lifetime_ of all to ensure that the returned reference lives long
enough:

```rust
fn gimme_bigger<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let s1 = "kekega".to_string();     // --------+- 'a
    let res;                           //         |
    {                                  //         |
        let s2 = "bur".to_string();    // --+- 'b |
        res = gimme_bigger(&s1, &s2);  //   |     | res has lifetime 'b
    }                                  // --+     | 
    // println!("bigger is {}", res);  //         | nope 😿 'b is invalid
}                                      // --------+ 
```

The `res` reference has the shorter lifetime `'b`. When used in the `println!`
macro, `'b` already out of scope, and so is `res`, the program will not compile.

Lifetime-parameters can also be used with _mutable references_:

- `&i32` - a reference
- `&'a i32` - a reference with a lifetime-parameterr
- `&'a mut i32` - a _mutable_ reference with a lifetime parameter

## Subtyping

_Subtyping_ is a type of compile-time polymorphism that check whether operations
working with a _supertype_ of type `T`, say `F<T>`, can operate also on type `T`
.

In the context of lifetimes, if lifetime `'a` lives _longer_ than lifetime `'b`
then `'a` is a _subtype_ of `'b`.

## Elision

Rust has a set of _lifetime elision_ rules built into the compiler for cases
when a function's parameter and return value's lifetime-parameters can be
inferred.

The first rule states that all _input references_ get their own
lifetime-parameter:

```rust
fn print_longest<'a, 'b>(s1: &'a str, s2: &'b str) {
    println!("{}", if s1.len() > s2.len() { s1 } else { s2 });
}

// elides to
fn print_longest(s1: &str, s2: &str) {
    println!("{}", if s1.len() > s2.len() { s1 } else { s2 });
}
```

The second rule is that when the function takes a _single reference_ as an
argument and _returns references_, the returned references' lifetime is the same
as the input reference's lifetime:

```rust
fn first_half<'a>(s: &'a str) -> &'a str {
    let h = s.len() / 2;
    &s[..h]
}

// elides to
fn first_half(s: &str) -> &str {
    let h = s.len() / 2;
    &s[..h]
}
```

The third rule states that when a method that takes `&self` returns references,
the returned references' lifetime is the same as the lifetime of `&self`, even
if it also takes other references:

```rust
struct Text<'a> {
    text: &'a str,
}

impl<'a> Text<'a> {
    fn print_other_get<'b>(self: &'a Self, s: &'b str) -> &'a str {
        println!("other is {}", s);
        self.text
    }
    // elides to
    fn print_other_get(&self, s: &str) -> &str {
        println!("other is {}", s);
        self.text
    }
}
```

This works only if it returns a reference from `&self`. Returning a different
reference needs explicit lifetime-parameters:

```rust
struct Text<'a> {
    text: &'a str,
}

impl<'a> Text<'a> {
    fn or_longer(&self, s: &'a str) -> &'a str {
        if self.text.len() > s.len() {
            self.text
        } else {
            s
        }
    }
}
```

## Static

The `'static` lifetime is the _subtype_ of all other lifetimes — it lives for
the entire duration of the program.

The most notable example is _string literals_, whose full type
is `&'static str`:

```rust
fn gimme_bigger<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let s1 = "kekega";                 // &'static str
    let res;
    {
        let s2 = "bur";                // &'static str
        res = gimme_bigger(&s1, &s2);  // res has lifetime 'static
    }
    println!("bigger is {}", res);     // ok 🎉
}
```

Other references `'static` lifetimes are ones created in the _global scope_
declared as `static`.
