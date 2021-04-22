use std::slice;

fn main() {
    // Dereference a raw pointer
    println!("> Dereference a raw pointer");
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize; // random location in memory
    let r_yolo = address as *mut i32;
    // would crash:
    // let slice_yolo: &[i32] = unsafe { slice::from_raw_parts_mut(r_yolo, 10000) };

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Call an unsafe function or method
    println!("> Call an unsafe function or method");
    unsafe {
        dangerous();
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Access or modify a mutable static variable
    println!("> Access or modify a mutable static variable");

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    println!("name is: {}", HELLO_WORLD);
    // TODO

    // Implement an unsafe trait
    println!("> Implement an unsafe trait");
    // No code here

    // Access fields of unions
    println!("> Access fields of unions");
    let u = MyUnion { f1: 1 };
    let f = unsafe { u.f1 };
}

unsafe fn dangerous() {}

// split_at works ok because references are immutable
fn split_at(slice: &[i32], mid: usize) -> (&[i32], &[i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&slice[..mid], &slice[mid..])
}

// split_at_mut has to use unsafe code for this to work
// the function is "smarter as the compiler", since the
// two slices don't overlap, even though they are from
// the same original slice
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// May be duplicated across the binary:
const HELLO_WORLD_CONST: &str = "Hello, world!";
// Has static memory address and can be mutable:
static HELLO_WORLD: &str = "Hello, world!";
static mut HELLO_WORLD_MUT: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// https://doc.rust-lang.org/reference/items/unions.html
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}
