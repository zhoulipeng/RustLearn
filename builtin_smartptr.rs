//https://en.wikipedia.org/wiki/Rust_(programming_language)#Examples
//A demonstration of Rust's built-in unique smart pointers, along with tagged unions and methods:

use IntList::{Node, Empty};

// This program defines a recursive data structure and implements methods upon it.
// Recursive data structures require a layer of indirection, which is provided here
// by a unique pointer, constructed via the `Box::new` constructor. These are
// analogous to the C++ library type `std::unique_ptr`, though with more static
// safety guarantees.
fn main() {
    let list = IntList::new().prepend(3).prepend(2).prepend(1);
    println!("Sum of all values in the list: {}.", list.sum());
    println!("Sum of all doubled values in the list: {}.", list.multiply_by(2).sum());
}

// `enum` defines a tagged union that may be one of several different kinds of values at runtime.
// The type here will either contain no value, or a value and a pointer to another `IntList`.
enum IntList {
    Node(i32, Box<IntList>),
    Empty
}

// An `impl` block allows methods to be defined on a type.
impl IntList {
    fn new() -> Box<IntList> {
        Box::new(Empty)
    }

    fn prepend(self, value: i32) -> Box<IntList> {
        Box::new(Node(value, Box::new(self)))
    }

    fn sum(&self) -> i32 {
        // `match` expressions are the typical way of employing pattern-matching,
        // and are somewhat analogous to the `switch` statement from C and C++.
        match *self {
            Node(value, ref next) => value + next.sum(),
            Empty => 0
        }
    }

    fn multiply_by(&self, n: i32) -> Box<IntList> {
        match *self {
            Node(value, ref next) => Box::new(Node(value * n, next.multiply_by(n))),
            Empty => Box::new(Empty)
        }
    }
}
