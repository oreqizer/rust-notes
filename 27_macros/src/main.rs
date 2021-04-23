fn main() {
    macro_rules_vec();
}

// Types:
// 1. declarative macros with macro_rules!
// 2. three kinds of procedural macros (custom #[derive], attribute-like and function-like)

// NOTE:
// current macro_rules! will be deprecated and replaced with
// declarative macros 2.0: https://github.com/rust-lang/rust/issues/39412

// Declarative macros
// the most basic ones, the default when people refer to macros
#[macro_export]
macro_rules! vec2 {
    // similar to 'match' expressions. patterns are matched, followed by =>
    // that generates the code for the matched pattern
    //
    // the first ( ) encapsulates the whole pattern
    // the $( ) matches the value used in the parentheses
    // the ,* indicates that the $( ) can repeat 0..n times
    // $x:expr matches an inner expression
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                // $( )* repeats as many times as the $( ) got matched
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn macro_rules_vec() {
    // which brackets are used doesn't matter (but { } get a formatted space before them):
    let _v = vec2!(1, 2, 3);
    let _v = vec2![1, 2, 3];
    let v = vec2! {1, 2, 3};
    println!("vector v: {:?}", v);
}

// Procedural macros 🙀
// TODO
