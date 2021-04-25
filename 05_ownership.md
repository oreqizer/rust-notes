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

### Copying

When a primitive value is assigned to another variable or passed to a function,
its value is _copied_. This copy has its own scope in the function:

```rust
let n1 = 5;
let n2 = n1; // n1 copied to n2, both valid

gimme_string(n2);
// both n1 and n2 valid

fn gimme_number(n: i32) {  // n copied, has the scope of the function
    println!("Got a number: {}", n);
}                          // nothing special happens
```

## Allocation

When a dynamic memory is allocated, a _fat pointer_ to this data is stored
into a variable.

> A _fat pointer_ is a pointer containing additional metadata, like length
> and capacity.

When this variable goes out of scope, the memory is freed:

```rust
fn print_kek() {
    let s1 = String::from("hello");  // s1 points to the allocated data
    println!("{}", s1);
}                                    // s1 goes out of scope, data freed
```

This is how a `String` fat pointer looks like:

![String pointer](./assets/string_ptr.svg)

### Moving

When a fat pointer is assigned to a variable, the variable becomes the pointer's
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

When returning fat pointers from functions, their ownership is moved:

```rust
let s = make_string(); // s owns the return value of make_string()

fn make_string() -> String {
    String::from("lmao")
}
```

## References

Reference to a value is taken by the `&` operator. This creates a _thin pointer_
to the data. Taking a reference to a value is called _borrowing_.

```rust
let s = String::from("kek");
println!("Length is {}", string_length(&s));

fn string_length(s: &String) -> usize {
    s.len()
}
```

References are _immutable_ by default. Use `&mut` to make them _mutable_:

```rust
let mut s = String::from("top");
append_kek(&mut s); // s is now topkek

fn append_kek(s: &mut String) {
    s.push_str("kek");
}
```

Note that only mutable variables can be borrowed as mutable.

Reference to a fat pointer to a value stored on the heap:

![Reference to a fat pointer](./assets/string_ptr_ref.svg)

References must obey the following rules:
- there can be any number of _immutable_ references
- there can be only one _mutable_ reference
- when a _mutable_ reference exists, no _immutable_ reference can exist

Mutating primitive values is done by _dereferencing_ using the `*` operator:

```rust
let mut n = 10;
to_five(&mut n); // n is now 5

fn to_five(x: &mut i32) {
    *x = 5; // changes the value at the reference location to 5
}
```

References are valid until the end of their _scope_:

```rust
fn task() {
    let number = 5;
    let x = &number;
    gimme_number(&x);
    // x still valid
    let y = x + 5;
    gimme_number(&y);
}   // scope ends, x no longer valid

fn gimme_number(n: &i32) {  // number borrowed here
    println!("Thanks, got {}", n);
}                           // reference still valid
```

Immutable references are no longer valid after a _mutable borrow_:

```rust
let number = 5;
let x = &number;     // valid
let y = &number;     // valid
println!("Numbers: {}, {}", x, y);

let z = &mut number; // x and y no longer valid!
println!("Number: {}", z);
```

References whose values were created inside a function cannot be returned
from within the function. Functions can only return created _primitives_
and _fat pointers_:

```rust
// OK, value copied
fn gimme_primitive() -> i32 {
    1337
}

// OK, ownership moved
fn gimme_pointer() -> String {
    String::from("yep")
}

// Error!
fn bad_function() -> &String {
    let s = String::from("nope");
    &s // compilation error!
}  // s no longer valid here, cannot return from the function
```
