use std::{thread, time};

/// `FizzBuzz` using sructured style
///
/// This is a very simple example using "C-ike" structured style.
pub fn using_structured_style() {
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

/// `FizzBuzz` showing Rust if which is an expression
///
/// All 'if' statements in Rust are expressions.
/// Learn to appreciate them or perish.
pub fn using_if_as_expression() {
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

/// `FizzBuzz` showing OOP style using composition.
///
/// Rust doesn't have inhertance, but uses "interfaces"
/// called traits in Rust
pub fn using_oop_style() {
    let obj = FizzBuzz::new("Fizz".to_string(), "Buzz".to_string());
    for number in 1..101 {
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
