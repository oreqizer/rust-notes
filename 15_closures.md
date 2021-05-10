# Closures

Anonymous functions that can _capture their environment_:
- by reference `&T`
- by mutable reference `&mut T`
- by value `T`

The preference goes from top-to-bottom — if possible borrow by immutable or
mutable reference, and only capture by value when really needed.

```rust
fn main() {
    let var = 4;                // var declared here
    let add_var = |x| x + var;  // closure captures var

    println!("{} + 3 = {}", var, add_var(3));
}
```

If the closure only contains a single expression, the block brackets `{}` can
be left out. Closures can be type-annotated as regular functions, but types are
inferrable:

```rust
fn main() {
    let squared = |x: i32| -> i32 { x * x };  // full syntax
    let squared = |x: i32| -> i32 x * x ;     // no block
    let squared = |x| { x * x };              // inferred type (more flexible)
    let squared = |x| x * x;                  // inferred type and no block
}
```

## Borrowing

When only immutable values are captured, and they're not _consumed_, they're
borrowed by reference:

```rust
fn main() {
    let var = 4;
    let add_var = |x| x + var;  // var is &var here

    println!("{} + 3 = {}", var, add_var(3));
}
```

Mutable values are captured as `&mut T` and the closure needs to be stored
as `mut` as well:

```rust
fn main() {
    let mut count = 0;
    let mut inc_print = || {  // is mut because it mutates the environment
        count += 1;           // count is &mut count here
        println!("count is {}", count);
    };

    inc_print();
    inc_print();
}
```

Values that do not implement the `Copy` trait and are consumed in the closure
get their ownership moved:

```rust
fn main() {
    let a = "hello".to_string();  // String does not implement Copy
    let fs = || {
        let b = a;                // a moved into b
        println!("we got {}", b);
    };
    // println!("a is {}", a);    // nope 🙀 a is moved
}
```

If `a` was `i32`, this code would work, because `i32` implements `Copy`, and `a`
would get captured as `&a`. The `b` variable would get a copy of the value.

## Arguments

Closures are the most useful as _arguments to functions_ that perform certain
action:

```rust
fn do_twice<T>(f: impl Fn(T) -> T, a: T) -> T {
    f(f(a))
}

fn main() {
    println!("do add twice with x = 5: {}", do_twice(|x| x + x, 5));
}
```

## Keyword `move`

The `move` keyword transforms captured variabled by _reference_ or _mutable
reference_ to _owned by value_:

```rust
use std::ops::Add;

fn make_adder<T: Add<Output = T> + Copy>(x: T) -> impl Fn(T) -> T {
    move |y| x + y
}

fn main() {
    let add_5 = make_adder(5);
    println!("5 + 10 = {}", add_5(10));
}
```

## The `Fn` family

There are _three_ traits in the `Fn` family:
- `Fn` with the `call` method that takes `&self`
- `FnMut` (supertrait of `Fn`) with the `call_mut` method that takes `&mut self`
- `FnOnce` (supertrait of `FnMut`) with the `call_once` method that takes `self`

Closures that capture by _reference_ implement the `Fn` trait. Closures that capture
by _mutable reference_ implement `FnMut` and allow mutating the environment.
Closures capturing by _value_ implement `FnOnce`, because once they're called, the
values are moved, and cannot be called again.

Closures are actually implemented as _structs_ created at compile time. The
captured environment becomes the struct's fields. The created struct implements
the proper `Fn` trait.

Implementing a pseudo-closure struct implementing the `Fn` trait would look
something like this:

```rust
// captured environment
struct Adder<'a> {
    x: &'a i32,
}

// quasi Fn impl
impl<'a> Adder<'a> {
    fn call(&self, y: i32) -> i32 {
        self.x + y
    }
}

fn main() {
    let x = 5;
    let f = Adder { x: &x };                // capture the environment
    println!("{} + 3 = {}", x, f.call(3));  // would f(3) with a regular closure
}
```

A similar example for `FnMut` also shows why closures that mutate the environment
_must_ be marked as `mut` — the environment is a mutable `struct`:

```rust
// captured mutable environment
struct Counter<'a> {
    count: &'a mut i32,
}

// quasi FnMut impl
impl<'a> Counter<'a> {
    fn call_mut(&mut self) {
        *self.count += 1;
        println!("count is {}", self.count)
    }
}

fn main() {
    let mut count = 0;
    let mut counter = Counter {
        count: &mut count,  // capture the mutable environment
    };
    counter.call_mut();     // would counter() with a regular closure
    counter.call_mut();     // would counter() with a regular closure
}
```

Finally, showing how the `FnOnce` trait consumes the environment:

```rust
// captured owned environment
struct Spawner {
    s: String,
}

// quasi FnOnce impl
impl Spawner {
    fn call_once(self) {
        let a = self.s;
        println!("spawn a thread or something with {}", a);
    }
}

fn main() {
    let s = "yolo".to_string();
    let spawn = Spawner { s };
    spawn.call_once();          // would spawn() with a regular closure
    // spawn.call_once();       // nope 🙀 spawner is moved
    // println!("s is {}", s);  // nope 🙀 s is moved
}
```
