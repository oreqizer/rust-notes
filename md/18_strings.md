# Strings

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

## Characters

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

## Indexing

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

## Formatting

While concatenating can be done using the `+` operator, a more readable way that
also allows more complex formatting is using the `format!` macro:

```rust
fn main() {
    let s = format!("{}, {}!", "hello", "world");
}
```

_TODO_
