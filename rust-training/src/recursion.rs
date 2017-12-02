use tuple;
use std::{thread, time};

/// PRIVATE: Example using function call recursion.
///
/// Pretty much the most basic recursion example where
/// we init the recusrsion via function call with a count argument.
fn using_recursion_and_tuples(number: u8) {
    println!(">>>> [{}] using_recursion_and_tuples", file!());
    let sleep_duration = time::Duration::from_millis(100);
    println!("{}", tuple::use_match_with_tuples(number));
    thread::sleep(sleep_duration);
    if number < 100 {
        using_recursion_and_tuples(number + 1);
    }
}

/// PUBLIC: Run all recursion examples.
pub fn run_recursion() {
    using_recursion_and_tuples(1);
}
