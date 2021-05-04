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

## Defaults

The type can be defaulted using the `T=Default` syntax, overwritten using `::<>`,
aka the **turbofish**:

```rust
struct Point<T=u64>(T, T);

fn main() {
    let p = Point(13, 37);        // u64
    let p = Point::<i32>(4, 20);  // i32
}
```

## Implementation bounds

Methods on generic types can be implemented only for certain concrete types,
creating _trait bounds_ only the implemented types satisfy:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn sum(&self) -> i32 {
        self.x + self.y
    }
}

fn main() {
    let p1 = Point { x: 13, y: 37 };
    let p2 = Point { x: "lol", y: "kek" };

    println!("sum = {}", p1.sum());     // ok
    // println!("sum = {}", p2.sum());  // nope!
}
```
