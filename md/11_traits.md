# Traits

Traits give types _behaviour_ via methods they implement, such as:

* `Add` trait allows the use of `+`
* `PartialOrd` makes types comparable
* `Display` enables automatic formatting

> The **orphan rule** states that either the _trait_, or the _implementor_
> have to be internal types. _External traits_ cannot be implemented on
> _external types_.

They are defined using the `trait` keyword, and a list of methods a type should
implement:

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

An alternative `where` syntax exists for when there are too many generic
parameters with many trait bounds:

```rust
fn some_function<T, U>(t: &T, u: &U)
    where
        T: Display + Clone,
        U: PartialOrd + Debug,
{
    // ...
}
```

### Method implementation

Trait bounds can be used for conditionally implementing methods on generic types
in case the concrete type satisfies the bounds:

```rust
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

Methods can also be conditionally implemented for concrete types:

```rust
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Pair<i32> {
    fn sum(&self) -> i32 {
        self.x + self.y
    }
}
```

## Trait objects

The `dyn Trait` syntax allows specifying _trait objects_ — dynamic objects that
implement the trait's behavior. They consist of two pointers:

- pointer to the actual data
- pointer to the object's _virtual method table_

Only _object-safe_ traits can be used in trait objects. The trait's methods must
follow these rules:

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

Trait objects are mainly useful when it is impossible to use an `enum`, like
when the number of possible types satisfying the trait is unknown.

## Type `impl Trait`

The `impl Trait` type annotation can be used in function _arguments_ and as a
_return_ type. It allows specifying unnamed, but concrete types that implement a
trait:

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

The `impl Trait` syntax can be used when returning values from functions.
Contrary to using `Box<dyn Trait>`, this does not cause the value to be stored
on the heap:

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

## Fully qualified syntax

The _fully qualified syntax_ for function calls in the context of traits is:

- `Trait::function(receiver, args..)` for methods
- `<Type as Trait>::function(args..)` for associated functions

A struct can implement a method that collides with the name of an implemented
trait's method. Calling an associated function on the _trait_ with the struct as
the _receiver_ calls the trait implementation:

```rust
trait Pilot {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

fn main() {
    let h = Human;
    Pilot::fly(&h);  // This is your captain speaking.
    h.fly();         // *waving arms furiously*
}
```

The full form would be `<Human as Pilot>::fly(&h);`, but since Rust knows that
`&h` is of type `&Human`, it knows which implementation of the `Pilot` trait to
call.

When a struct's and a trait's associated function names collide, the struct has
to be cast to the trait using `as` to call the trait's implementation:

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("{}", Dog::baby_name());              // Spot
    println!("{}", <Dog as Animal>::baby_name());  // puppy
}
```

The short form `Animal::baby_name()` cannot be called, because Rust cannot infer
the concrete implementation.
