use std::collections::HashMap;

fn main() {
    // Task 1
    let res = task_1(&vec![1, 3, 3, 7, 4, 2, 0]);
    println!("Task 1: {:?}", res);

    // Task 2
    println!("Task 2 — 'first' -> {}", task2("first"));
    println!("Task 2 — 'apple' -> {}", task2("apple"));

    // 3.
    // Using a hash map and vectors, create a text interface to allow a user
    // to add employee names to a department in a company. For example, “Add Sally
    // to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
    // all people in a department or all people in the company by department, sorted
    // alphabetically.
}

#[derive(Debug)]
struct Task1Result {
    mean: f64,
    median: i32,
    mode: i32,
}

// 1.
// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position), and mode (the value that
// occurs most often; a hash map will be helpful here) of the list
fn task_1(list: &Vec<i32>) -> Task1Result {
    let len = list.len();
    let mut sorted = list.clone();
    sorted.sort();

    let mean = (list.iter().sum::<i32>() as f64) / (len as f64);
    let median = sorted[((len + 1) / 2)];
    let mut map: HashMap<i32, i32> = HashMap::new();
    for &i in list {
        let count = map.entry(i).or_insert(1);
        *count += 1;
    }
    let mut mode: i32 = 0;
    let count = 0;
    for (c, i) in map {
        if c > count {
            mode = i;
        }
    }

    Task1Result { mean, median, mode }
}

// 2.
// Convert strings to pig latin. The first consonant of each word is moved
// to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple”
// becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
fn task2(word: &str) -> String {
    let mut woveled = false;
    for c in "aeiouy".chars() {
        if word.starts_with(c) {
            woveled = true;
            break;
        }
    }

    let mut res = String::from(word);
    let mut postfix = String::from("");
    if woveled {
        postfix.push('h');
    } else {
        res.drain(..1);
        postfix.push(word.chars().next().unwrap());
    }

    format!("{}-{}ay", res, postfix)
}
