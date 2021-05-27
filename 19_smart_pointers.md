# Smart pointers

Smart pointers are basically _fat pointers_ with additional capabilities, like
all the containers like `Vec<T>` or `String`, as well as `Box<T>`, `Rc<T>`
and `RefCell<T>`.

These pointers implement the `Deref` and `Drop` traits. `Deref` is useful for
automatic dereferencing, so the types can be used both as references and
utilise _deref coersion_, and as pointers themselves. The `Drop` trait frees
memory, and optionally contains additional cleanup logic.

## Box

Values are allocated on the _stack_ by default. Putting them into a `Box<T>`
stores them on the _heap_:

```rust
fn main() {
    let x = 5;            // on the stack
    let y = Box::new(5);  // on the heap
}
```

Using `Box<T>` is necessary when working with _DSTs_, like slices
or `dyn Trait`:

```rust
use std::fmt::Display;

fn gimme_displayable(num: bool) -> Box<dyn Display> {
    Box::new(if num { 1337 } else { String::from("yo") })
}
```

## Rc

The `Rc<T>` type is used when multiple ownership is needed. The acronym stands
for _reference counting_.

The `clone` method creates a _strong_ clone that contribute to the
_strong count_ of references. The number of strong clones can be checked using
`Rc::strong_count`. When the strong count reaches zero, the value is dropped:

```rust
use std::rc::Rc;

fn main() {
    let x = Rc::new(5);  // Rc<i32>
    let y = x.clone();   // Rc<i32>
    let z = x.clone();   // Rc<i32>
    println!("strong count: {}", Rc::strong_count(&x)); // x, y, and z, so 3
}
```

The `Rc::downgrade` method can be used when working with potential reference
cycles. It creates a `Weak<T>` and increases the _weak count_. The difference is
that the weak count can be non-zero for the value to be dropped:

```rust
use std::rc::Rc;

fn main() {
    let x = Rc::new(5);         // Rc<i32>
    let y = x.clone();          // Rc<i32>
    let z = Rc::downgrade(&x);  // Weak<i32>
    println!("strong count: {}", Rc::strong_count(&x)); // x and y, so 2
    println!("strong count: {}", Rc::weak_count(&x));   // z, so 1
}
```

`Weak<T>` is especially useful when dealing with graphs or double linked lists
where reference cycles are common.

### Arc

_TODO_

## RefCell

_TODO_
