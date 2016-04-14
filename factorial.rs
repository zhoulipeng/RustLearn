// The branches in this function exhibit Rust's optional implicit return
// values, which can be utilized where a more "functional" style is preferred.
// Unlike C++ and related languages, Rust's `if` construct is an expression
// rather than a statement, and thus has a return value of its own.
fn recursive_factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * recursive_factorial(n - 1)
    }
}

fn iterative_factorial(n: u32) -> u32 {
    // Variables are declared with `let`.
    // The `mut` keyword allows these variables to be mutated.
    let mut i = 1u32;
    let mut result = 1u32;
    while i <= n {
        result *= i;
        i += 1;
    }
    return result; // An explicit return, in contrast to the prior function.
}

fn iterator_factorial(n: u32) -> u32 {
    // Iterators have a variety of methods for transformations.
    // |accum, x| defines an anonymous function.
    // Optimizations like inline expansion reduce the range and fold
    // to have performance similar to iterative_factorial.
    (1..n + 1).fold(1, |accum, x| accum * x)
}

fn main() {
    println!("Recursive result: {}", recursive_factorial(10));
    println!("Iterative result: {}", iterative_factorial(10));
    println!("Iterator result: {}", iterator_factorial(10));
}
