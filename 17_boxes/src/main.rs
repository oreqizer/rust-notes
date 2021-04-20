use std::ops::Deref;

fn main() {
    // Box is essentially a "fat pointer" AFAIK
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Deref coersion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // -> hello(&(*m)[..]);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Enables dereference operator "*"
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// Fust a silly function
fn hello(name: &str) {
    println!("Hello, {}!", name);
}