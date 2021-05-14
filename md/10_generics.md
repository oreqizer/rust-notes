# Generics

Generics allow specifying placeholders for _concrete types_:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

The compiler substitutes these placeholders for concrete types on compile time.

## Turbofish

Specifying _type parameters_ explicitly on generic types is done with the
**turbofish** syntax `::<>`:

```rust
fn main() {
    let v = (1..10).collect::<Vec<u32>>();
}
```

## Defaults

The type can be defaulted using the `T=Default` syntax:

```rust
struct Point<T = u64>(T, T);

fn main() {
    let p = Point(13, 37);        // u64
    let p = Point::<i32>(4, 20);  // i32
}
```

## Associated type

Associated types are a type of generics whose purpose is to simplify code
management.

> Code using the associated type can be replaced with code using the
> generic type, but not the other way around.

Associated type is specified using `type` in the `impl` block and can be
accessed with `::`:

```rust
trait Graph {
    type N;
    type E;
    fn has_edge(&self, start: &N, end: &N) -> bool;
}

fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> uint {
    // ...
}
```

The same defined using generics is a lot less readable:

```rust
trait Graph<N, E> {
    fn has_edge(&self, start: &N, end: &N) -> bool;
}

fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> uint {
    // ...
}
```

Associated types can be defaulted, allowing both flexibility and clean syntax:

```rust
trait Add<Rhs = Self> {
    type Output = Rhs;
    fn add(&self, rhs: Rhs) -> Self::Output;
}

struct Meters(u32);

struct Millimeters(u32);

impl Add for Meters {
    type Output = Meters;

    fn add(self, rhs: Meters) -> Meters {
        Meters(self.0 + rhs.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}
```
