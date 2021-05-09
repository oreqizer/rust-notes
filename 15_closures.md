# Closures

Anonymous functions that can _capture their environment_:

```rust
fn main() {
    let var = 4;                // var declared here
    let add_var = |x| x + var;  // closure captures var

    println!("{} + 3 = {}", var, add_var(3));
}
```

When a closure _closes over_ its environment, all variables the closure uses
are considered _borrowed_ and cannot be changed:

```rust
fn main() {
    let mut var = 4;
    let add_var = |x| x + var;

    // var = 5;  // nope 🙀 var is borrowed
    println!("{} + 3 = {}", var, add_var(3));
}
```

_TODO_ https://medium.com/coding-rust/best-explanation-of-closure-in-rust-2b20210eba53
