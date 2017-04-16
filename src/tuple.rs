/// PUBLIC: Run all tuple examples.
pub fn run_tuple() {
    using_structured_style();
    using_match_with_tuples();
}

/// PUBLIC: Reusable function using tuple unpacking.
///
/// This function can be called from other modules also.
pub fn use_match_with_tuples(number: u8) -> String {
    let mut retval = String::new();
    match (number % 3, number % 5) {
        (0, 0) => retval.push_str("FizzBuzz"),
        (0, _) => retval.push_str("Fizz"),
        (_, 0) => retval.push_str("Buzz"),
        (_, _) => retval.push_str(&number.to_string()),
    }
    retval
}

/// PRIVATE: Example using structure style with tuples.
///
/// This example shows how to access tuples.
fn using_structured_style() {
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

/// PRIVATE: Example using tuples via a "match" block.
///
/// This is a nice example of tuple unpacking.
/// Code is very readable and easy to understand.
/// In Rust "match" is preferred over "if-elif-else" structure.
fn using_match_with_tuples() {
    println!(">>>> [{}] using_match_with_tuples", file!());
    for number in 1..101 {
        use_match_with_tuples(number);
    }
}
