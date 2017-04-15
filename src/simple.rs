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
    fn three(&self) -> &String {
        // TODO: make mutable
        //self.fizz = "Fizz".to_string();
        return &self.fizz;
    }
}
//trait IsDivisableBy {
//fn three(&self, number: u8) -> &String;
//fn five(&self, number: u8) -> String;
//}

//impl IsDivisableBy for FizzBuzz {
//fizz = "Fizz".to_string();
// fn three(&self, number: u8) -> &String {
//     if number % 3 == 0 {
//         "Fizz".to_string()
//     }
//     &self.fizz
// }

//fn five(&self, number: u8) -> String {}
//}

pub fn using_oop_style() {
    for number in 1..101 {}
}
