fn main() {
    enum_initializer();
}

enum Status {
    Value(u32),
    Stop,
}

// TODO add this to enums
fn enum_initializer() {
    // enum initializers are functions that return their instance
    // and can be used as such:
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// TODO add this to closures
// and mention that impl Fn works instead!
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    // Box needed because it implements the Sized trait
    // and that is needed for function arguments/return values
    Box::new(|x| x + 1)
}
