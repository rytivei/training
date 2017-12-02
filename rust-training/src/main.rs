mod simple;
mod tuple;
mod recursion;
mod vector;

fn main() {
    simple::run_simple();
    tuple::run_tuple();
    recursion::run_recursion();
    vector::run_vector();
    // TODO: filesystem.rs
    // TODO: mmap.rs
    // TODO: networking.rs
    // TODO: threads.rs
}
