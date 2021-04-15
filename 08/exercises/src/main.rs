use std::collections::HashMap;
use std::io;

fn main() {
    // Task 1
    // let res = task_1(&vec![1, 3, 3, 7, 4, 2, 0]);
    // println!("Task 1: {:?}", res);

    // Task 2
    // println!("Task 2 — 'first' -> {}", task2("first"));
    // println!("Task 2 — 'apple' -> {}", task2("apple"));

    // Task 3
    task3();
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

// 3.
// Using a hash map and vectors, create a text interface to allow a user
// to add employee names to a department in a company. For example, “Add Sally
// to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
// all people in a department or all people in the company by department, sorted
// alphabetically.
enum Command {
    Add(String, String),
    List(Option<String>),
    Quit(),
}

impl Command {
    fn parse(input: &String) -> Option<Command> {
        let parts = input.trim().split_whitespace().collect::<Vec<&str>>();
        if parts.len() == 1 {
            if parts[0] == "List" {
                return Some(Command::List(None));
            }
            if parts[0] == "Quit" {
                return Some(Command::Quit());
            }
            return None;
        }
        if parts.len() == 2 {
            if parts[0] == "List" {
                let department = String::from(parts[1]);

                return Some(Command::List(Some(department)));
            }
            return None;
        }
        if parts.len() == 4 {
            if parts[0] != "Add" && parts[2] != "to" {
                return None
            }
            let person = String::from(parts[1]);
            let department = String::from(parts[3]);

            return Some(Command::Add(department, person));
        }
        None
    }
}

fn task3() {
    println!("Company register! Available commands:");
    println!("  1. Add <who> to <company>");
    println!("  2. List");
    println!("  3. Quit");

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if let Some(cmd) = Command::parse(&line) {
            match cmd {
                Command::Add(department, person) => {
                    company.entry(department).or_insert(Vec::new()).push(person);
                }
                Command::List(None) => {
                    for (dep, vec) in company.iter() {
                        for person in vec {
                            println!("{} ({})", person, dep);
                        }
                    }
                }
                Command::List(Some(department)) => {
                    if let Some(vec) = company.get(&department) {
                        for person in vec {
                            println!("{}", person);
                        }
                    } else {
                        println!("No such department");
                    }
                }
                Command::Quit() => {
                    break;
                }
            }
        } else {
            println!("Invalid command! Try again");
            println!("  1. Add <who> to <company>");
            println!("  2. List");
            println!("  3. Quit");
        }
    }
}
