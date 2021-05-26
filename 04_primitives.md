# Primitives

Rust has a wide variety of primitive values.

## Scalars

* signed ints `i8`, `i16`, `i32`, `i64`, `i128` and `isize` (pointer size)
* unsigned ints `u8`, `u16`, `u32`, `u64`, `u128` and `usize` (pointer size)
* floats `f32`, `f64`
* `char` **Unicode** scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each)
* `bool` either `true` or `false`
* unit type `()`, which is an empty tuple

### Literals

Integers `420`, floats `13.37`, chars `'a'`, strings `"kek"`, booleans `true`
and the union type `()` can be expressed as literals.

Integers can have a prefix to change their expression:

- `0x` hexadecimals
- `0o` octals
- `0b` binary

Numbers can have a suffix like `100u32` to set their type. Numeric literals can
have `_` placed in them for readability, like `1_000_000`.

Operators and their precedence is similar to **C-like** languages:

```rust
fn main() {
    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
```

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
