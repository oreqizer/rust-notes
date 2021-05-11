# Collections

Rust has a _lot_ of built-in collection data types. The most commonly used ones
are `Vec<T>` and `HashMap<K, V>`. All collections are located in the
`std::collections` module.

The special `String` collection is for string manipulation.

## Vectors

The `Vec<T>` type's an ordered collection of items with a variable length:

```rust
fn main() {
    let mut v = Vec::new();

    v.push(1337);
    v.push(420);
}
```

### Initialization

A new zero-length vector is created using the `Vec::new()` associated function.
To instantiate a vector with values, use the `vec!` macro:

```rust
fn main() {
    let v = vec!["a", "bunch", "of", "string", "refs"];
}
```

To allocate space beforehand, use `Vec::with_capacity(usize)`.

### Reading

There's a _safe_, and an _unsafe_ way to get a value at a specific index.

Using the `Index` trait will _panic_ when accessing an element beyond the
vector's size. It is only recommended using in situations where the element is
guaranteed to exist:

```rust
fn main() {
    let v = vec![1, 2, 3];

    let second = v[1];     // ok
    // let fourth = v[3];  // boom 💥
}
```

The safe way is using the `get` method, which returns `Option<T>`:

```rust
fn main() {
    let v = vec![1, 2, 3];

    let second = v.get(1); // Some(2)
    let fourth = v.get(3); // None
}
```

## Hash maps

Key-value pairs are most commonly stored in `HashMap<K, V>`. Any type that
implements the `Eq` and `Hash` trait can be used as the key:

```rust
use std::collections::HashMap;

fn main() {
    let mut fav_nums = HashMap::new();

    fav_nums.insert("Bobby", "1337");
    fav_nums.insert("Michael", "420");
}
```

Hash maps are initialized using `HashMap::new()` for an empty one, or
`HashMap::with_capacity(usize)` with pre-allocated space. Hash maps shrink when
they occupy too much space after removing elements.

### Entries

Normally, values are retrieved using the `get` method, which returns an
`Option<T>`. Using `entry` returns the `Entry` enum, which is either _occupied_
or _vacant_.

The `Entry` enum allows performing useful operations, such as `or_insert`:

```rust
fn main() {
    let mut users: HashMap<u32, &str> = HashMap::new();
    // ...
    let leet = users.entry(1337).or_insert("Bobby");
    // leet is now either an existing entry, or "Bobby" was inserted and returned
}
```

### Iteration

Hashmaps implement the `Iterable` trait and produce a sequence of `(K, V)`
tuples:

```rust
fn main() {
    let mut fav_nums = HashMap::new();

    fav_nums.insert("Bobby", "1337");
    fav_nums.insert("Michael", "420");
    // ...

    fav_nums.iter().for_each(|(name, number)| {
        println!("{}'s favorite number is {}", name, number);
    });
}
```

## Strings

_TODO_
