use std::{thread, time};

/// PUBLIC: Run all simple examples.
///
/// In Rust it doesn't matter where you declare your function inside a file.
pub fn run_simple() {
    using_structured_style();
    using_if_as_expression();
    using_oop_style();
}

/// PRIVATE: Example using C-like sructured style
///
/// This is the most basic example and very understandable to any coming from C language.
fn using_structured_style() {
    println!(">>>> [{}] using_structured_style", file!());
    let sleep_duration = time::Duration::from_millis(100);
    for number in 1..101 {
        thread::sleep(sleep_duration);
        if number % 15 == 0 {
            println!("FizzBuzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
    }
}

/// PRIVATE: Example using 'if' as a expression.
///
/// In Rust all 'if' statements are expressions, so you can use that to your advantage.
fn using_if_as_expression() {
    println!(">>>> [{}] using_if_as_expression", file!());
    let sleep_duration = time::Duration::from_millis(100);
    for number in 1..101 {
        thread::sleep(sleep_duration);
        let n = number.to_string();
        let value = if number % 15 == 0 {
            "FizzBuzz"
        } else if number % 3 == 0 {
            "Fizz"
        } else if number % 5 == 0 {
            "Buzz"
        } else {
            &*n
        };
        println!("{}", value);
    }
}

struct FizzBuzz {
    fizz: String,
    buzz: String,
}

impl FizzBuzz {
    // analoguous to "constructor"
    fn new(a_fizz: String, a_buzz: String) -> FizzBuzz {
        FizzBuzz {
            fizz: a_fizz,
            buzz: a_buzz,
        }
    }
    // borrowed
    fn three(&self) -> &String {
        &self.fizz
    }
    // borrowed
    fn five(&self) -> &String {
        &self.buzz
    }
    // ownership change
    fn fifteen(&self) -> String {
        let mut tmp = String::new();
        tmp.push_str(&self.fizz);
        tmp.push_str(&self.buzz);
        tmp
    }
}

/// PRIVATE: Example showing OOP style using composition.
///
/// Rust doesn't have inheritance, because it uses the more flexible composition paradigm.
pub fn using_oop_style() {
    println!(">>>> [{}] using_oop_style", file!());
    let sleep_duration = time::Duration::from_millis(100);
    let obj = FizzBuzz::new("Fizz".to_string(), "Buzz".to_string());
    for number in 1..101 {
        thread::sleep(sleep_duration);
        if number % 15 == 0 {
            println!("{}", obj.fifteen());
        } else if number % 3 == 0 {
            println!("{}", obj.three());
        } else if number % 5 == 0 {
            println!("{}", obj.five());
        } else {
            println!("{}", number);
        }
    }
}
