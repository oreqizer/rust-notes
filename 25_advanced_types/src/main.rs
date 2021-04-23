fn main() {
    type_alias();
    type_alias_2();
}

// Type alias
type Kilometers = i32;

fn type_alias() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn type_alias_2() {
    takes_long_type(returns_long_type());
}

fn takes_long_type(f: Thunk) {
    // --snip--
    f();
}

fn returns_long_type() -> Thunk {
    // --snip--
    let f: Thunk = Box::new(|| println!("hi"));
    f
}

// The ! type that never returns
// called diverging functions
fn loophole() -> ! {
    // --snip--
    loop {
        println!("fuck, am stuck");
    }
}

fn never_type() -> i32 {
    let o: Option<i32> = Some(1337);

    // Formally, the ! type can be coerced into any other type
    match o {
        Some(x) => x,
        None => loophole(),
    }
}

// Dynamically Sized Types (DST) and the Sized Trait
fn generic<T>(t: T) {
    // --snip--
}

// gets transformed automatically to
fn generic2<T: Sized>(t: T) {
    // --snip--
}

// to allow non-Sized generics, this special Sized syntax is needed:
fn generic3<T: ?Sized>(t: &T) {
    // --snip--
}

