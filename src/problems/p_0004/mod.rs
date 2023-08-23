//! **Problem 4** - *Largest Palindrome Product*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        4,
        "Largest Palindrome Product",
        solve,
    )
}


use crate::shared::numbers::is_palindrome;

fn solve() -> String {
    let mut largest_palindrome: u128 = 0;

    for fact1 in 100..1000 {
        for fact2 in fact1..1000 {
            let product = fact1 * fact2;
            if is_palindrome(product) && (product > largest_palindrome) {
                largest_palindrome = product;
            }
        }
    }

    largest_palindrome.to_string()
}