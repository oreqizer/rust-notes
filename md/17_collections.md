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

The _slice string_ type `&str` points to a **UTF-8** valid collection of bytes
of type `&[u8]`.

Tye `String` type is a UTF-8 valid wrapper around `Vec<u8>` that contains
utility methods for string manipulation:

```rust
fn main() {
    let mut s = String::new();

    for c in "Hello".chars() {
        s.push(c);
    }
    s.push_str(", world!");

    println!("{}", s);
}
```

### Characters

The `char` type is a _4-byte_ primitive type that holds a single _Unicode code
point_. These code points form _graphemes_, either individually, or as
_grapheme clusters_:

```rust
fn main() {
    let chars: &[char] = &['न', 'म', 'स', '्', 'त', 'े'];
    let graphemes = ["न", "म", "स्", "ते"]; //'स', '्' makes "स्", 'त', 'े' makes "ते"
}
```

Individual string characters can be iterated using the `chars` method:

```rust
fn main() {
    for c in "नमस्ते".chars() {
        println!("{}", c); // prints न म स ् त े
    }
}
```

Individual characters take up more space than strings, because `char` is always
4-bytes in size, compared to many string characters being 1 to 3-bytes in size.

### Indexing

Indexing strings is thus ambiguous, because it is not clear whether _bytes_ or
_chars_ are being indexed. For this reason, indexing strings is done explicitly
via:

- `.chars().nth(i)` for _chars_
- `.bytes().nth(i)` for _bytes_

```rust
fn main() {
    let ciao = "Здравствуйте";

    // prints 12 characters
    for i in 0..ciao.chars().count() {
        println!("ciao.chars().nth({}) = {}", i, ciao.chars().nth(i).unwrap());
    }

    // prints 24 bytes
    for i in 0..ciao.len() {
        println!("ciao.bytes().nth({}) = {}", i, ciao.bytes().nth(i).unwrap());
    }
}
```

Note that the `len` method returns the number of _bytes_ of a string, not
_chars_.

### Formatting

While concatenating can be done using the `+` operator, a more readable way that
also allows more complex formatting is using the `format!` macro:

```rust
fn main() {
    let s = format!("{}, {}!", "hello", "world");
}
```
