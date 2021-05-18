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

## Escaping

The ``\`` character is used for escaping. To write a literal ``\``, it has to be
escaped with `\\`. String or character literal delimeters within a literal must
be escaped:

```rust
fn main() {
    println!("backslash: \\");
    println!("chars: {}", '\'');
    println!("strings: {}", "\"");
}
```

Escaping can be used for writing _bytes_ by their hexadecimal value, or _Unicode
code points_:

```rust
fn main() {
    println!("how about \x74\x68\x65\x20\x67\x61\x6d\x65");  // bytes
    println!("Unicode char U+211D is \u{211D}");             // Unicode
}
```

Escaping allows writing _multiline strings_ with escaped whitespace:

```rust
fn main() {
    let s = "Did your \
    mother fuck \
    a snowman?";

    println!("{}", s);
}
```

### Raw strings

Useful when no escaping at all is desired. They can be declared using `r""` and
optionally an arbitrary number of `#` pairs outside of `""`, depending on
whether `"` is in the string and how many `#` characters are used within the
string:

```rust
fn main() {
    let raw = r"nope: \u{211D}, nope: \x67\x61\x6d\x65";
    let raw = r#"even more "nope" here"#;
    let raw = r###"nope #nope ##nope"###;
}
```

### Byte strings

Strings of bytes that are mostly text are created using `b""` and are stored
as an array of type `[u8; N]`:

```rust
fn main() {
    let bytes = b"raw bytes amirite?"; // type &[u8; 18]
}
```

They allow escaping the same way as regular strings, except for Unicode code
points:

```rust
fn main() {
    let bytes = b"the \x67\x61\x6d\x65 again lmao";  // ok
    // let bytes = b"nope \u{211D}";                 // nope 🙀
}
```

Byte strings don't have to be a valid UTF-8:

```rust
use std::str;

fn main() {
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

    match str::from_utf8(shift_jis) {
        Ok(s) => println!("Like that's ever going to happen: {}", s),
        Err(e) => println!("Told ya: {}", e),
    };
}
```

They can be made _raw_ the same way as regular strings:

```rust
fn main() {
    let rbs = br##"hashtag #raw "strings" amirite?"##; // type &[u8; 31]
}
```

## Formatting

Concatenating can be done using the `+` operator:

```rust
fn main() {
    let s = "top".to_string();
    println!("{}", s + "kek"); // topkek
}
```

More complex formatting can be done using the `format!` macro:

```rust
fn main() {
    let s = format!("{}, {}!", "hello", "world");
}
```

### Styles

The formatting syntax has the form `{<position>:<format>}`, both parts being
optional. When none are supplied also the `:` can be omitted. It is verified at
compile-time.

The `<position>` part can be the argument position, or a named argument:

```rust
fn main() {
    println!("Rofl {}", "lmao");           // implicit position
    println!("Rofl {0}", "lmao");          // explicit position
    println!("Rofl {arg}", arg = "lmao");  // named position
}
```

The `<format>` part determines which trait to use when formatting:

- nothing for `Display`
- `?` for `Debug`
- `o` for `Octal`
- `x` for `LowerHex`
- `X` for `UpperHex`
- `p` for `Pointer`
- `b` for `Binary`
- `e` for `LowerExp`
- `E` for `UpperExp`

```rust
fn main() {
    println!("{:?}", 1337);  // debug
    println!("{:b}", 1337);  // binary
    println!("{:X}", 1337);  // upper-case hexadecimal

    println!("1337 = {leet:X}, 420 = {:?}", 420, leet = 1337);  // mishmash
}
```

Further traits can be added in the future.
