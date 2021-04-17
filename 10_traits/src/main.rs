use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Blanket impl:
// impl<T: Display> ToString for T {
//     // --snip--
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Trait: Clone -> Copy
fn largest_2<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        let item = item.clone();
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Return: T -> &T
fn largest_3<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = list[0];

    let mut index = 0;
    for (i, &item) in list.iter().enumerate() {
        if item > largest {
            largest = item;
            index = i;
        }
    }

    &list[index]
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
