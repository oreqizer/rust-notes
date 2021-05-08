# Lifetimes

Lifetimes ensure that _references_ point to a valid memory. Lifetime
of a reference is as big as is the _scope_ of a variable:

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

_Lifetime_ denotes scope, _lifetime-parameter_ denotes parameters that
the compiler substitutes with a real lifetime, same as inferring types
of generics.

Lifetime-parameters are named `'a`, an apostrophe and a lowercase name,
often just a single letter, notable exception being `'static`. As generics,
they're declared using the `<>` syntax:

```rust
fn gimme_bigger<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

In this case, references `s1` and `s2` may have different lifetimes, and
it is impossible to determine the returned reference's lifetime at compile
time.

When a lifetime-parameter is used on multiple references, the compiler
choses _the shortest lifetime_ of all to ensure that the returned reference
lives long enough:

```rust
fn gimme_bigger<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let s1 = String::from("kekega");   // --------+- 'a
    let res;                           //         |
    {                                  //         |
        let s2 = String::from("bur");  // --+- 'b |
        res = gimme_bigger(&s1, &s2);  //   |     | res has lifetime 'b
    }                                  // --+     | 
    // println!("bigger is {}", res);  //         | nope 😿 'b is invalid
}                                      // --------+ 
```

The `res` reference has the shorter lifetime `'b`. When used in the `println!`
macro, `'b` already out of scope, and so is `res`, the program will not compile.

## Subtyping

_Subtyping_ is a type of compile time polymorphism that check whether operations
working with a _supertype_ of type `T`, say `F<T>`, can operate also on type `T`.

In the context of lifetimes, if lifetime `'a` lives _longer_ than lifetime `'b`
then `'a` is a _subtype_ of `'b`.

## Elision

_TODO_

## Static

The `'static` lifetime is the _subtype_ of all other lifetimes.

_TODO_
