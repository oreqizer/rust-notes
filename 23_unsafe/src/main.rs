fn main() {
    // Dereference a raw pointer
    println!("> Dereference a raw pointer");
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize; // random location in memory
    let r_yolo = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Call an unsafe function or method
    println!("> Call an unsafe function or method");
    // TODO

    // Access or modify a mutable static variable
    println!("> Access or modify a mutable static variable");
    // TODO

    // Implement an unsafe trait
    println!("> Implement an unsafe trait");
    // TODO

    // Access fields of unions
    println!("> Access fields of unions");
    // TODO
}
