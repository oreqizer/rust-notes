# References

Reference to a value is taken by the `&` operator. This creates a _thin pointer_
to the data. Taking a reference to a value is called _borrowing_.

```rust
fn main() {
    let s = "kek".to_string();
    println!("Length is {}", string_length(&s));
}

fn string_length(s: &String) -> usize {
    s.len()
}
```

Reference to a fat pointer to a value stored on the heap:

![Reference to a fat pointer](assets/string_ptr_ref.svg)

References are _immutable_ by default. Only mutable variables can be borrowed as
mutable. Use `&mut` to make a reference _mutable_:

```rust
fn main() {
    let mut s = "top".to_string();
    append_kek(&mut s); // s is now topkek
}

fn append_kek(s: &mut String) {
    s.push_str("kek");
}
```

References must obey the following rules:

- there can be any number of _immutable_ references
- there can be only one _mutable_ reference
- when a _mutable_ reference exists, no _immutable_ reference can exist

## Scope

Same as variables, references are valid until the end of their _scope_.
Immutable references are no longer valid after a _mutable borrow_:

```rust
fn main() {
    let mut number = 5;
    let x = &number;     // valid
    let y = &number;     // valid
    println!("Numbers: {}, {}", x, y);

    let z = &mut number; // x and y no longer valid!
    println!("Number: {}", z);
}
```

References whose values were created inside a function cannot be returned from
within the function. Functions can only return created _primitives_ and _fat
pointers_:

```rust
// OK, value copied
fn gimme_primitive() -> i32 {
    1337
}

// OK, ownership moved
fn gimme_pointer() -> String {
    "yep".to_string()
}

// Error!
fn bad_function() -> &String {
    let s = "nope".to_string();
    &s // compilation error!
}  // s no longer valid here, cannot return from the function
```

## Dereferencing

Mutating referenced values is done by _dereferencing_ using the `*` operator:

```rust
fn main() {
    let mut n = 10;
    to_five(&mut n); // n is now 5
}

fn to_five(x: &mut i32) {
    *x = 5; // changes the value where the reference points at to 5
}
```

Note that the reference must be a `&mut`.

### Deref coercion

A compiler feature called `Deref` coercion automatically dereferences types
implementing the `Deref` trait if the supplied type to a function call doesn't
match the expected type.

The supplied type is being _dereferenced_ until the suitable type is found:

```rust
fn gib_num(x: &i32) {
    // ...
}

fn main() {
    let x = Box::new(5);
    gib_num(&*x);   // is the explicit form
    gib_num(&x);    // turns into &*x
    gib_num(&&&x);  // turns into &*&*&*x
}
```

This feature is most commonly used by _fat pointers_ and _slices_.
