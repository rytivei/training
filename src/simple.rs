use std::{thread, time};

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
    fn new(a_fizz: String, a_buzz: String) -> FizzBuzz {
        FizzBuzz {
            fizz: a_fizz,
            buzz: a_buzz,
        }
    }
    // getter
    fn three(&self) -> &String {
        &self.fizz
    }
    // getter
    fn five(&self) -> &String {
        &self.buzz
    }
    fn fifteen(&self) -> String {
        let mut tmp = String::new();
        tmp.push_str(&self.fizz);
        tmp.push_str(&self.buzz);
        tmp
    }
}
//trait IsDivisableBy {
//fn three(&self, number: u8) -> &String;
//fn five(&self, number: u8) -> String;
//}

pub fn using_oop_style() {
    let obj = FizzBuzz::new("Fizz".to_string(), "Buzz".to_string());
    for number in 1..101 {
        let n = number.to_string();
        obj.three();
        obj.five();
        obj.fifteen();
        println!("{}", n);
    }
}
