# Ownership

Rust's ownership system follows these rules:
- Each value has a variable that’s called its _owner_
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

Values have to have a _fixed size_ known at _compile time_ to be stored
on the _stack_.

Variable- or dynamically-sized data are stored on the _heap_, and their
_pointer_ is stored on the stack.

## Scope

Variable bindings have a _scope_, and are constrained to live in a _block_.
Variables are valid as long as their scope is valid:

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward
    // ...
}                      // this scope is now over, and s is no longer valid
```

## Allocation

When a dynamic memory is allocated, the _pointer_ to this data is stored
into a variable. When this variable goes out of scope, the memory is freed:

```rust
fn print_kek() {
    let s1 = String::from("hello");  // s1 points to the allocated data
    println!("{}", s1);
}                                    // s1 goes out of scope, data freed
```

This is how a `String` pointer looks like:

![String pointer](./assets/string_ptr.svg)

### Moving

When a pointer is assigned to a variable, the variable becomes the pointer's
_owner_. If the pointer is assigned to another variable or passed to a function,
the pointer is _moved_ and gets a new owner:

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 moved to s2, s1 is no longer valid

gimme_string(s2);
// cannot use s2 anymore :(

fn gimme_string(s: String) {  // s moved, now owned by the function
    println!("Now I own: {}", s);
}                             // s no longer valid and deallocated
```
