/// `FizzBuzz` with tuples using structure style
///
/// Very basic example of using tuples.
/// Almost so simple it hurts my eyes :)
pub fn using_structured_style() {
    println!(">>>> [{}] using_structured_style", file!());
    for number in 1..101 {
        let mut pair: (String, String) = (String::from(""), String::from(""));
        if number % 15 == 0 {
            pair.0 = "Fizz".to_string();
            pair.1 = "Buzz".to_string();
        } else if number % 3 == 0 {
            pair.0 = "Fizz".to_string();
            pair.1 = "".to_string();
        } else if number % 5 == 0 {
            pair.0 = "".to_string();
            pair.1 = "Buzz".to_string();
        } else {
            pair.0 = number.to_string();
            pair.1 = "".to_string();
        }
        println!("{}{}", pair.0, pair.1);
    }
}

/// `FizzBuzz` using tuples via a "match" block.
///
/// This is a nice example of tuple unpacking.
/// Code is very readable and easy to understand.
/// Try to use "match" instead of if-elif-else structures in Rust.
pub fn using_match_with_tuples() {
    println!(">>>> [{}] using_match_with_tuples", file!());
    for number in 1..101 {
        match (number % 3, number % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", number),
        }
    }
}
