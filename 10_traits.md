# Traits

Traits give types _behaviour_ via methods they implement, such as:
* `Add` trait allows the use of `+`
* `PartialOrd` makes types comparable
* `Display` enables automatic formatting

They are defined using the `trait` keyword, and a list of methods a type
should implement:

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

Traits are then implemented as on specific types in an `impl` block that
specifies the interface, and the type after the `for` keyword:

```rust
struct Point(i32, i32);

impl Summary for Point {
    fn summarize(&self) -> String {
        format!("Point({}, {})", self.0, self.1)
    }
}
```

Traits can have a default implementation:

```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("Summarized...")
    }
}

struct Point(i32, i32);

impl Summary for Point {} // default implementation used
```

Referencing the _implementor type_ is done via the `Self` type:

```rust
trait Creator {
    fn duplicate() -> Self;
    fn equals(other: &Self) -> bool;
}
```

Traits can also define _associated functions_:

```rust
trait Spawner {
    fn new() -> Self;
}

struct Point(i32, i32);

impl Spawner for Point {
    fn new() -> Self {
        Point(0, 0)
    }
}
```

## Trait bounds

To restrict generic parameters to only allow types with certain functionality,
_trait bounds_ can be specified:

```rust
fn largest<T: PartialOrd>(s: &[T]) -> Option<&T> {
    s.iter().reduce(|acc, x| if x > acc { x } else { acc })
}
```

The `PartialOrd` trait allows comparing values. The `T` generic parameter here
is restricted to only allow slices of comparable types.

Multiple trait bounds can be combined using `+`:

```rust
fn print_largest<T: PartialOrd + Display>(s: &[T]) {
    let res = s.iter().reduce(|acc, x| if x > acc { x } else { acc });
    
    if let Some(l) = res {
        println!("largest is {}", l);
    }
}
```

An alternative `where` syntax exists for when there are too many generic parameters
with many trait bounds:

```rust
fn some_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: PartialOrd + Debug,
{
    // ...
}
```

## Trait objects

The `dyn Trait` syntax allows specifying _trait objects_ — dynamic objects that implement
the trait's behavior. They consist of two pointers:
- pointer to the actual data
- pointer to the object's _virtual method table_

Only _object-safe_ traits can be used in trait objects. The trait's methods must follow
these rules:
- the return type is not `Self`
- there are no generic type parameters

```rust
trait Draw {
    fn draw(&self);
}

struct Button;
struct Select;

impl Draw for Button {
    fn draw(&self) {}
}

impl Draw for Select {
    fn draw(&self) {}
}

fn main() {
    let ui: Vec<Box<dyn Draw>> = vec![
        Box::new(Button),
        Box::new(Select),
    ];

    for el in ui.iter() {
        el.draw();
    }
}
```

Trait objects are mainly useful when it is impossible to use an `enum`, like when the number
of possible types satisfying the trait is unknown.

## Type `impl Trait`

The `impl Trait` type annotation can be used in function _arguments_ and as a 
_return_ type. It allows specifying unnamed, but concrete types that implement
a trait:

```rust
trait Trait {}

fn foo(arg: impl Trait) {}

fn bar() -> impl Trait {}
```

### Arguments

When used as an argument, it is almost the same as using a _generic_ parameter
with a _trait bound_:

```rust
trait Trait {}

fn foo<T: Trait>(arg: T) {}

fn bar(arg: impl Trait) {}
```

The only difference is that the generic syntax allows specifying the type `T`
using the turbofish syntax as with `foo::<usize>(1)`.

### Return

The `impl Trait` syntax can be used when returning values from functions. Contrary
to using `Box<dyn Trait>`, this does not cause the value to be stored on the heap:

```rust
trait Trait {}

impl Trait for i32 {}

fn blazeit() -> impl Trait {
    420
}
```

The main usage of this pattern is with _closures_, as they don't have a specific
type, they only satisfy the `Fn` family of traits:

```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
```

## Supertraits

Traits can require implementors to also implement other traits, becoming their 
_supertraits_:

```rust
trait Mlg: Display {
    fn mlg(&self) -> String {
        format!("xxx_{}_xxx", self)
    }
}
```

Multiple trait implementations can be required by joining traits with `+`:

```rust
trait Mlg: Display + PartialOrd {
    fn winner(&self, other: &Self) -> String {
        if self > other {
            format!("xxx_{}_xxx", self)
        } else {
            format!("blazeit_{}", other)
        }
    }
}
```

## Associated type

_TODO_

## Fully qualified syntax

_TODO_
