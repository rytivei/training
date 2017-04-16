use tuple;

/// PRIVATE: Example using function call recursion.
fn using_recursion_and_tuples(number: u8) {
    println!(">>>> [{}] using_recursion_and_tuples", file!());
    println!("{}", tuple::use_match_with_tuples(number));
    if number < 100 {
        using_recursion_and_tuples(number + 1);
    }
}

/// PUBLIC: Run all recursion examples.
pub fn run_recursion() {
    using_recursion_and_tuples(1);
}
