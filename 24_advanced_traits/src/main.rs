use std::fmt;
use std::ops::Add;

fn main() {
    // Operator overload
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // Qualified syntax for method calls on traits
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

// Associated type
// https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        None
    }
}

// If the iterator was a generic, it would allow multiple
// implementations of the trait. Calling the next() funciton
// would then require specifying the implementation we want
// to use all the time.
pub trait IteratorGen<T> {
    fn next(&mut self) -> Option<T>;
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Default generic parameter:
// trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

// impl Add<Self> for Point {
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// Different Add type other than Self has to be specified explicitly
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// Associated trait functions without &self
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Using Supertraits to Require One Trait’s Functionality Within Another Trait
trait OutlinePrint: fmt::Display {
    // OutlinePrint requires Display trait implementation
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    // implement Display first
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// Using the Newtype Pattern to Implement External Traits on External Types
// this pattern is useful for abstracting implementation details
// or extending functionality
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
