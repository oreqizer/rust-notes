# Control flow

Most of the control flow constructs except `for` and `while` loops are _expressions_.

## Block

Blocks create their own _scope_. They are an _expression_ with the value of
the last expression in the block:

```rust
fn main() {
    let x = {
        let y = 5;
        println!("y = {}", y);
        y
    };
    println!("x = {}", x);
}
```

Note that the last expression cannot be followed by `;` or it would be considered
a _statement_ instead.

## If

Works like in most languages, except is an _expression_:

```rust
fn main() {
    let ternary = if x < 5 { "small" } else { "big" };
}

fn rng(x: i32) -> i32 {
    if x < 5 {
        x
    } else if x < 10 {
        x * 2
    } else {
        x * 3
    }
}
```

## Match

A switch-like expression that matches patterns and values. Must be _exhaustive_ —
all possible patterns and values must be matched at least in one arm:

```rust
fn main() {
    let text = match val {
        5 => "five",
        x => "some other number",
    };
}
```

### If let

Syntax sugar for `match` that matches one pattern does nothing with the rest:

```rust
fn main() {
    if let Some(x) = val {
        println!("x = {}", x);
    }
}
```

Can also have `else` and `else if` like regular `if` expressions.

## Loop

Loops forever. The `break` statement can be called with a value to return a value
from the `loop`.

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
}
```

## For

Looping through iterables is done with a `for` statement:

```rust
fn main() {
    for number in 1..4 {
        println!("{}", number);
    }
}
```

## While

Runs `while` the condition evaluates to `true`:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}
```
