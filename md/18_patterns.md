# Patterns

Patterns can be matched and destructured in various ways:

```rust
fn main() {
    let t = (1, 3, 37);
    
    let r = match t {
        (420, ..) => "blazeit",
        (0, 0, 0) => "unit",
        _ => "other stuff",
    };
}
```

## Destructuring

_TODO_ mention that available in variable assignments, function arguments
https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring.html

_TODO_ literals, named variables, stuff like `..`

### Tuples

_TODO_

### Enums

_TODO_

### Structs

_TODO_

### Pointers

_TODO_

## Guards

_TODO_

## Binding

_TODO_
