# Primitives

Rust has a wide variety of primitive values.

## Scalars

* signed ints `i8`, `i16`, `i32`, `i64`, `i128` and `isize` (pointer size)
* unsigned ints `u8`, `u16`, `u32`, `u64`, `u128` and `usize` (pointer size)
* floats `f32`, `f64`
* `char` **Unicode** scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each)
* `bool` either `true` or `false`
* unit type `()`, which is an empty tuple

## Tuples

A fixed-size collection of values:

```rust
fn main() {
    let vector = (1, 3, 3.7);
    let single = (420, ); // trailing comma needed for single element tuples
}
```

Types can differ:

```rust
fn main() {
    let mishmash = (420, "blazeit");
}
```

They're indexed with `.` and a number, e.g. `.0` for the first element. Their
type can be specified in arguments:

```rust
fn multiply_vector(v: (i32, i32, i32), n: i32) -> (i32, i32, i32) {
    (v.0 * n, v.1 * n, v.2 * n)
}
```

## Arrays

A collection of objects of the same type. Their length is known at _compile
time_. Their signature is `[T; length]`.

```rust
fn main() {
    let xs = [1, 2, 3, 4, 5]; // type [i32; 5]
}
```

Creating an array with certain length and a default value:

```rust
fn main() {
    let xs = [0; 500]; // type [i32, 500]
}
```
