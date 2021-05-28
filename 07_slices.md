# Slices

Slices are a type of _references_ that reference contiguous sequence of elements
in a collection rather than the whole collection:

```rust
fn main() {
    let s = "hello world".to_string();

    let hello = &s[..5];
    let world = &s[6..];
}
```

This is how slices are stored in memory:

![Slice in memory](assets/string_slice.svg)

## Range

Ranges are defined using the `..` syntax for exclusive, and `..=` for inclusive
range. The first or last index can be dropped to capture the start or end of a
collection:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let s = &a[0..5];   // [1, 2, 3, 4, 5]
    let s = &a[0..=4];  // [1, 2, 3, 4, 5]
    let s = &a[..];     // [1, 2, 3, 4, 5]
    let s = &a[1..];    // [   2, 3, 4, 5]
    let s = &a[..4];    // [1, 2, 3, 4   ]
    let s = &a[2..=3];  // [      3, 4   ]
}
```

## Collections

More complex collections such as `Vec<T>` or `String` use primitive collections
such as arrays and `str` data under the hood. They mainly serve as an
abstraction for data manipulation using methods such as `Vec::push`
or `String::push_str`.

### Dereferencing

Data types that use primitive collections under the hood implement the `Deref`
trait so that they return a slice of the underlying data when sliced:

```rust
fn main() {
    let a = [1, 2, 3, 4];
    let v = vec![1, 2, 3, 4];

    let s1 = &a[..]; // &[i32]
    let s2 = &v[..]; // &[i32]
    // s1 == s2
}
```

### Arguments

When a funciton does an _immutable borrow_ of a collection, the best practice is
to use slices as arguments instead of a specific collection implementation. This
allows passing both raw values and more abstract data types:

```rust
fn main() {
    let s = after_n(3, "lolkekbur");               // &str string literal slice directly
    let s = after_n(3, &"lolkekbur".to_string());  // Deref-coerced &String -> &str

    let (x, y) = split_half(&[1, 2, 3, 4]);        // &[i32] array slice directly
    let (x, y) = split_half(&vec![1, 2, 3, 4]);    // Deref-coerced &Vec<T> -> &[i32]
}

fn after_n(n: usize, s: &str) -> &str {
    &s[n..]
}

fn split_half(a: &[i32]) -> (&[i32], &[i32]) {
    let len = a.len() / 2;
    (&a[..len], &a[len..])
}
```

`Deref` coercion _slices_ collections that wrap slices:

```rust
fn main() {
    let word = "lolkekbur".to_string();

    let s = after_n(3, &word[..]);  // is the explicit form
    let s = after_n(3, &word);      // turns into &word[..]
    let s = after_n(3, &&&word);    // turns into *&*&&word[..]
}

fn after_n(n: usize, s: &str) -> &str {
    &s[n..]
}
```

