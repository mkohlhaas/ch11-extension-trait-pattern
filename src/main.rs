// ============================= //
// 1. Define the extension trait //
// ============================= //

use std::fmt::Debug;

// We require `Iterator` so we have access to `.next()`.
// We require `Sized` because `print_all` takes `self` by value.
pub trait IteratorExt: Iterator + Sized {
    /// Prints every remaining item in the iterator to the console.
    fn print_all(self)
    where
        Self::Item: Debug,
    {
        for item in self {
            println!("{:?}", item);
        }
    }
}

// ================================================================ //
// 2. Implement the extension trait using a blanket implementation. //
// ================================================================ //

// This automatically grants `print_all` to ANY type that implements Iterator.
impl<I: Iterator> IteratorExt for I {}

fn main() {
    // ======================================== //
    // 3. Bring the extension trait into scope. //
    // ======================================== //

    // If you comment this line out, the code will not compile!
    use IteratorExt;

    {
        // Example A: Using it on a standard vector iterator
        println!("=== Printing Numbers ===");

        let numbers = vec![10, 20, 30];
        numbers.into_iter().print_all();
    }

    {
        // Example B: Using it with iterator adapters (like .map and .filter)
        println!("\n=== Printing Filtered Words ===");

        let words = vec!["apple", "banana", "cherry", "date"];

        words
            .into_iter()
            .filter(|word| word.starts_with('b') || word.starts_with('c'))
            .map(|word| word.to_uppercase())
            .print_all(); // Chained seamlessly at the end
    }
}
