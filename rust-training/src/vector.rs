use tuple;
use std::{thread, time};

/// PUBLIC: Run all vector examples.
pub fn run_vector() {
    using_vector_stupid();
    using_vector_with_consumer();
}

fn using_vector_stupid() {
    println!(">>>> [{}] using_vector_stupid", file!());
    //let mut vec = vec![];
    //for number in 1..101 {
    //vec.push(i);
    //}
    // actual fizzbuzz
}

/// PRIVATE: Example using a vector with a consumer.
///
/// TBD
fn using_vector_with_consumer() {
    println!(">>>> [{}] using_vector_with_consumer", file!());
    let sequence = (1..101).collect::<Vec<u8>>();
    for number in sequence.iter() {
        println!("{}", tuple::use_match_with_tuples(*number));
    }
}
