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

The _newtype_ pattern (name from **Haskell**) is wrapping a single type in a
struct. It is mainly useful for:
- implementing _external traits_ on _external types_
- enforcing type safety on more abstract types
- hiding implementation details

There's no runtime penalty, the wrapper is removed at compile time.

Avoiding the _orphan rule_ by wrapping `Vec<String>` and implementing `Display`:

```rust
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Vec[{}]", self.0.join(", "))
    }
}
```

Giving types more explicit names and implementing additional functionality
on them has the additional benefit of type safety:

```rust
use std::ops::Add;

struct Meters(u32);
struct Millimeters(u32);

impl Meters {
    fn to_millimeters(&self) -> Millimeters {
        Millimeters(self.0 * 1000)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}
```

Implementing units as `u32` would make keeping track of what is what extremely
difficult.

Wrapping more abstract types can also help hide implementation details:

```rust
struct Person;

struct People(HashMap<u32, Person>);

impl People {
    fn new() -> People {
        People(HashMap::new())
    }

    fn add(&mut self, id: u32, person: Person) {
        self.0.insert(id, person);
    }
}
```

Consumers of this code do not need to know that `People` is implemented as
a `HashMap`, which allows restricting the public API and makes refactoring
a lot easier.

## Never

The `!` stands for the _never_ type for functions that never return, meaning
they either loop forever or exit the program:

```rust
fn loophole() -> ! {
    loop {
        println!("RIP ☠️");
    }
}

fn int_or_bye(o: Option<i32>) -> i32 {
    // Formally, the ! type can be coerced into any other type
    match o {
        Some(x) => x,
        None => loophole(),
    }
}
```

One of the main functions that exit the program is the `panic!` macro.

## DSTs
