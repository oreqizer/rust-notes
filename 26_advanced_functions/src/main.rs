fn main() {
    fn_pointer();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn fn_pointer() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

enum Status {
    Value(u32),
    Stop,
}

fn enum_initializer() {
    // enum initializers are functions that return their instance
    // and can be used as such:
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    // Box needed because it implements the Sized trait
    // and that is needed for function arguments/return values
    Box::new(|x| x + 1)
}
