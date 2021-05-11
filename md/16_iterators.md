# Iterators

Types that implement the `Iterator` trait. The functional way of processing a
collection of items:

```rust
struct User {
    name: String,
    money: u32,
    active: bool,
}

fn average_active_users_money(users: &[User]) -> f64 {
    users
        .iter()
        .filter(|u| u.active)
        .map(|u| u.money as f64)
        .sum::<f64>()
        / users.len() as f64
}
```

## Definition

The `Iterator` trait defines one non-defaulted `next` method that returns the
iterator's next element, if any. The definition looks like this:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

The `next`'s method's responsibility is to:

- return the next element, if any
- mutate the iterator to advance on the next element

### Trait `IntoIterator`

Types that describe a collection commonly implement the `IntoIterator` trait
that defines how the type will be converted into an _iterator_:

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item=Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

Note that the `into_iter` method takes `self` as an argument, thus taking
ownership of the type that implements the trait.

### Trait `FromIterator`

In addition to `IntoIterator`, collection types can implement the `FromIterator`
trait to allow usage of the `.collect()` method:

```rust
trait FromIterator<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>;
}
```

The `IntoIterator` and `FromIterator` traits can be used in conjunction to
iterate over values and collect them back:

```rust
fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    let big_nums: Vec<_> = nums.into_iter().filter(|&x| x > 3).collect();
}
```

### In `for` loops

The `for` loop is actually a syntax sugar for iterators.
Implementing `IntoIterator`
allows usage in `for` loops:

```rust
fn main() {
    // the for loop
    let names = vec!["Bobby", "Michael", "Alaine"];
    for name in names {
        println!("{}", name);
    }

    // desugars into something like
    let names = vec!["Bobby", "Michael", "Alaine"];
    let mut iterator = names.into_iter();
    while let Some(name) = iterator.next() {
        println!("{}", name);
    }
}
```

### Conventions

The convention is to define and call methods:

- `into_iter(self)` for iterating over `T` via the `IntoIterator` trait
- `iter(&self)` for iterating over `&T` by _convention_
- `iter_mut(&mut self)` for iterating over `&mut T` by _convention_

Types commonly implement the `IntoIterator` trait **3-times**, once for each
variant. For example, `Vec<T>` implementations are:

- `impl<T> IntoIterator for Vec<T>`
- `impl<'a, T> IntoIterator for &'a Vec<T>` that calls `iter(&self)`
- `impl<'a, T> IntoIterator for &'a mut Vec<T>` that calls `iter_mut(&mut self)`

The first implementation on `Vec<T>` directly yields _by value_. The other two
are references themselves, so even though the `into_iter(self)` method takes
`self` by value, the value is actually a reference in both cases. They yield
_references_ to their content:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // Vec<T>, values copied or ownership moved
    for n in v {
        println!("copy of n = {}", n);
    }

    // &Vec<T>, values are &T
    for n in &v {
        println!("ref of &n = {}", n);
    }

    // &mut Vec<T>, values are &mut T
    let mut v = v;
    for n in &mut v {
        *n *= 2;
    }
}
```

Using the generic `into_iter` method directly **is context dependent**  and can
sometimes yield unexpected results. It is recommended calling `iter`
or `iter_mut` explicitly, if available.

## Methods

In addition to the mandatory `next` method, `Iterator` has a set of _consuming_
and _producing_ methods called _adaptors_.

Iterators are **lazy** — no adaptors get called until the iterator is actually
consumed by calling the `next` function.

### Consuming

A _consuming adaptor_ is one that consumes the iterator by calling `next` until
there are no more items to iterate over, such as `fold` or `sum`:

```rust
fn first_or_second(nums: &[i32]) -> (i32, i32) {
    nums.iter().fold(
        (0, 0),
        |(a, b), x| if a < b { (a + x, b) } else { (a, b + x) },
    )
}
```

### Producing

A method that produces a new iterator is called an _iterator adaptor_. They
perform transformations such as _mapping_, _filtering_, _zipping_, and others:

```rust
fn long_ass_function(a1: &[i32], a2: &[i32]) -> i32 {
    a1.iter()
        .zip(a2)
        .filter(|(&a, &b)| a > 0 && b > 0)
        .map(|(a, b)| a * b)
        .sum()
}
```

Check
the [official docs](https://doc.rust-lang.org/std/iter/trait.Iterator.html#provided-methods)
for the full list of functions, many are useful in everyday programming!
