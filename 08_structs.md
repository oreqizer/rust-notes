# Structs

Structs hold related data of different types together and have a name:

```rust
// Named fields struct
struct User {
    username: String,
    email: String,
    credits: u64,
}

// Tuple struct
struct Point(i32, i32, i32);

// Unit struct
struct Empty;
```

## Instances

Because Rust has no zero or nullish values, structs need to be instantiated with
all their fields filled:

```rust
fn main() {
    let user1 = User {
        username: "xxx_BILLY_xxx".to_string(),
        email: "blaze@michael.it".to_string(),
        credits: 0,
    };
}
```

A structure needs to be marked as `mut` to allow updating values:

```rust
fn main() {
    let mut user1 = User {
        username: "xxx_BILLY_xxx".to_string(),
        email: "blaze@michael.it".to_string(),
        credits: 0,
    };

    user1.email = "crash@team.racing".to_string();
}
```

Fields with the same name as an existing binding in the current scope can use
the shorthand syntax for field assignment:

```rust
fn main() {
    let username = "xxx_BILLY_xxx".to_string();
    
    let user1 = User {
        username, // username: username
        email: "blaze@michael.it".to_string(),
        credits: 0,
    };
}
```

Creating new structures from existing ones can be done using destructuring with
the `..` syntax:

```rust
fn main() {
    // ...

    let user2 = User {
        username: "_____samo_____".to_string(),
        ..user1
    };
}
```

## Methods

Methods on structs are defined using the `impl` block and can reference
themselves using `self` as the first parameter:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

Ownership rules apply for the `self` parameter, so it can be specified as an
immutable reference `&self` (most common), mutable reference `&mut self` or take
the ownership of the value when specified as `self`.

### Self

A special type `Self` is available in method definition that refers to the type
the method is defined on:

```rust
struct Rectangle {
    x: i32,
    y: i32,
}

impl Rectangle {
    fn square(side: i32) -> Self {
        Self { x: side, y: side };
    }
}
```

The `self` parameter notation is just a syntax sugar. The explicit forms look
like this:

- `self` is `self: Self`
- `&self` is `self: &Self`
- `&mut self` is `self: &mut Self`

### Automatic ref/deref

Rust's _automatic referencing and dereferencing_ feature allows uniform syntax
when calling methods on types regardless of the type of `self` and the value. It
adds `&`, `&mut` or `*` on the value before the method call as needed:

```rust
struct Point(i32, i32, i32);

impl Point {
    fn det(&self, other: &Point) -> i32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

fn main() {
    let p1 = Point(13, 37, 7);
    let p2 = Point(4, 2, 0);

    p1.det(&p2);
    // turns into
    (&p1).det(&p2);
}
```

## Associated functions

Associated functions are defined in an `impl` block and don't use the `self`
parameter. They are accessed on the structure name using `::`:

```rust
struct Point(i32, i32);

impl Point {
    fn new() -> Self {
        Point(0, 0)
    }
}

fn main() {
    let p = Point::new();
}
```

## Fully qualified syntax

The _fully qualified syntax_ for function calls of a type is:

- `Type::function(receiver_if_method, args..)`

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    v.push(4);
    // same as
    Vec::push(&mut v, 4);
}
```

Methods called with the full syntax allows the method to be used as a _function
pointer_.
