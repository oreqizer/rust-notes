# Types

The type system has various patterns and advanced constructs to be aware of.

## Type alias

Type aliases can be specified using the `type` keyword and used in-place of
the original type:

```rust
type ID = i32;

fn main() {
    let i: i32 = 1337;
    let id: ID = i;
}
```

They are mainly useful for creating an alias for more complex types to reduce
duplication and simplify code:

```rust
type Thunk = Box<dyn Fn(i32) -> i32>;

fn wrap_add(f: Thunk, a: i32) -> Thunk {
    Box::new(move |x| f(a) + a)
}
```

## Newtype

## Never

## DSTs
