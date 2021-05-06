# Enums

Enums define a type by enumerating its possible _variants_:

```rust
enum IpAddr {
    V4,
    V6,
}
```

Variants can hold various types of data:

```rust
enum Message {
    Quit,                        // no data
    Move { x: i32, y: i32 },     // anonymous struct
    Write(String),               // single type
    ChangeColor(i32, i32, i32),  // tuple
}
```

Variant constructors can be passed around as _function pointers_, since they are
functions that return their instance:

```rust
enum MaybeNum {
    None,
    Num(i32),
}

fn main() {
    let v: Vec<MaybeNum> = (1..=4).map(MaybeNum::Num).collect();
}
```

## Methods

Enums can define methods whose `self` parameter has the type of the enum:

```rust
enum Action {
    Quit,
    Message(String),
}

impl Action {
    fn call(&self) {
        // ...
    }
}
```

## Match

The `match` and `if let` expressions allow matching different `enum` variants:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    
    if let Coin::Penny = coin {
        println!("Penny!");
    }
}
```
