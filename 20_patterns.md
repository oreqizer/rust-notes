# Patterns

Patterns can be _matched_ and _destructured_ in various ways:

```rust
fn main() {
    let t = (1, 3, 37);
    let (x, y, z) = t;
    let r = match t {
        (420, ..) => "blazeit",
        (0, 0, 0) => "unit",
        _ => "other stuff",
    };
}
```

## Destructuring

Values can be destructured in `match`-like expressions and whenever they're
being assigned to a name, like _variable declaration_ or _function arguments_:

```rust
fn double_vec((x, y): (i32, i32)) -> (i32, i32) {
    (x * 2, y * 2)
}

fn main() {
    let v = (4, 2, 0);
    let (x, y, z) = v;
}
```

Literals can be matched directly or as a _range_, or bind to a name:

```rust
fn main() {
    let n = 5;
    match n {
        1337 => println!("leet"),
        0..=10 => prinln!("zero to ten"),
        x => println!("got a number {}", x),
    };
}
```

Bound literals can _shadow_ names:

```rust
fn main() {
    let n = Some(5);
    if let Some(n) = n {
        println!("got Some({})", n);
    };
}
```

A catch-all placeholder can be specified using `_`:

```rust
fn main() {
    let n = 5;
    match n {
        1..=10 => prinln!("one to ten"),
        _ => println!("some irrelevant number"),
    };
}
```

Destructuring is recursive:

```rust
fn main() {
    let x = Some(((13, 37), 15, "omg"));
    if let Some(((x, y), ..)) = x {
        println!("x = {}, y = {}", x, y);
    };
}
```

### Tuples

Tuples can be destructured into individual components. The lead or tail part of
a tuple can be ignored using the `..` syntax:

```rust
fn main() {
    let v = (1, 3, 7);
    match v {
        (4, 2, 0) => println!("blaze it"),
        (x, y, 0) => println!("2D vector {:?}", (x, y)),
        (1, ..) => println!("1 is first"),
        (.., 1) => println!("1 is last"),
        (x, y, z) => println!("x = {}, y = {}, z = {}", x, y, z),
    };
}
```

### Structs

Structs can be destructured to individual fields. Fields can be ignored using
the `..` syntax. A shorthand for field destructuring can be used when binding to
the same name as the field name:

```rust
struct Screen {
    width: i32,
    height: i32,
}

fn main() {
    let s = Screen {
        width: 1337,
        height: 420,
    };

    match s {
        Screen {
            width: 1920,
            height: 1080,
        } => println!("1080p"),
        Screen { width: 0..=480, .. } => println!("what a narrow screen"),
        Screen { width: 1000, height } => println!("width 1000 with height {}", height),
        Screen { width: w, height: h } => println!("{}x{}", w, h),
    };
}
```

### Pointers

References can be destructured into values:

```rust
fn main() {
    let n = &5;
    match n {
        &val => println!("n by value: {}", val),
    }
}
```

Values can be made into references using `ref` or `ref mut`:

```rust
fn main() {
    let s = "Hello".to_string();
    match s {
        ref s => println!("not moved! just ref'd: {}", s),
    }

    let mut s = s;
    match s {
        ref mut s => {
            s.push_str(", world!");
        }
    }
    println!("{}", s);
}
```

### Enums

Individual enum variants can be matched and further destructured:

```rust
enum Attribute {
    Empty,
    Color(u8, u8, u8),
    Text(String),
    Place { x: i32, y: i32 },
}

fn main() {
    let a = Attribute::Empty;
    match a {
        Attribute::Empty => println!("empty ????"),
        Attribute::Color(255, ..) => println!("something with red"),
        Attribute::Text(ref s) => println!("got text \"{}\"", s),
        Attribute::Place { x: 0, y: 0 } => println!("origin"),
        _ => println!("something different"),
    };
}
```

## Guards

Match _guards_ are created using the `if` keyword and a boolean expression:

```rust
fn main() {
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("same numbers"),
        (x, y) if x + y == 0 => println!("opposites"),
        (x, _) if x % 2 == 1 => println!("odd first"),
        _ => println!("something different"),
    }
}
```

## Or

Multiple patterns can be matched using `|`:

```rust
fn main() {
    let s = "lol";
    let cnd = false;
    match s {
        "lol" | "kek" | "bur" => println!("lmao"),
        "omg" | "wtf" if cnd => println!("conditional or"),
        _ => println!("shrug"),
    };
}
```

## Binding

Matched literals can be bound to a name using the `@` symbol:

```rust
fn main() {
    let n = 5;
    match n {
        x @ 0 => println!("zero"),
        x @ 1..=10 => println!("one to ten, got {}" x),
        _ => println!("something different"),
    };
}
```

## Unused names

Bound names starting with `_` are not checked as _unused_ by the compiler:

```rust
fn main() {
    let _x = 5; // whatever
}
```
