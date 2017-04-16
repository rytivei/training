mod simple;
mod tuple;
mod recursion;

fn main() {
    simple::run_simple();
    tuple::run_tuple();
    recursion::run_recursion();
    // TODO: filesystem.rs
    // TODO: mmap.rs
    // TODO: networking.rs
    // TODO: threads.rs
    // TODO: vector.rs
}
