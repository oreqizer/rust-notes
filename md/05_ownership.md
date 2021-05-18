# Ownership

Rust's ownership system follows these rules:

- Each value has a variable that’s called its _owner_
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

Values have to have a _fixed size_ known at _compile-time_ to be stored on the 
_stack_.

Variable- or dynamically-sized data are stored on the _heap_, and their
_pointer_ is stored on the stack.

## RAII

Rust enforces [RAII](https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization)
(Resource Acquisition Is Initialization). In addition to holding data on the
stack, variables also _own_ resources allocated on the heap.

Variable bindings have a _scope_, and are constrained to live in a _block_.
Variables are valid as long as their scope is valid:

```rust
fn main() {
    let w = "kek";
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        // ...
    }                      // this scope is now over, and s is no longer valid
    // w valid, s invalid
}
```

When a variable goes out of scope, its `Drop` trait destructor is called and its
resources are freed.

### Stack

When a primitive value is assigned to another variable or passed to a function,
its value is _copied_ and stored in the function's stack. This copy has its own
scope in the function:

```rust
fn main() {
    let n1 = 5;
    let n2 = n1; // n1 copied to n2, both valid

    gimme_string(n2);
    // both n1 and n2 valid
}

fn gimme_number(n: i32) {  // n copied, has the scope of the function
    println!("Got a number: {}", n);
}                          // nothing special happens
```

When a function ends, the function's stack data are popped.

### Heap

When a dynamic memory is allocated, a _fat pointer_ to this data is stored into
a variable.

> A _fat pointer_ is a pointer containing additional metadata, like length
> and capacity.

When this variable goes out of scope, its owned resources are freed:

```rust
fn print_kek() {
    let s1 = String::from("hello");  // s1 points to the allocated data
    println!("{}", s1);
}                                    // s1 goes out of scope, data freed
```

This is how a `String` fat pointer looks like:

![String pointer](../assets/string_ptr.svg)

## Moving

When a fat pointer is assigned to a variable, the variable becomes the pointer's
_owner_. If the pointer is assigned to another variable or passed to a function,
the pointer is _moved_ and gets a new owner:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved to s2, s1 is no longer valid

    gimme_string(s2);
    // cannot use s2 anymore :(
}

fn gimme_string(s: String) {  // s moved, now owned by the function
    println!("Now I own: {}", s);
}                             // s no longer valid and deallocated
```

When returning fat pointers from functions, their ownership is moved:

```rust
fn main() {
    let s = make_string(); // s owns the return value of make_string()
}

fn make_string() -> String {
    String::from("lmao")
}
```
