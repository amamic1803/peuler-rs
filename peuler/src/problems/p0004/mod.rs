use crate::Solution;
use pmath::digits::is_palindrome;

problem!(Problem0004, 4, "Largest Palindrome Product");

impl Solution for Problem0004 {
    fn solve(&self) -> String {
        let mut largest_palindrome: u64 = 0;

        for fact1 in 100..1000 {
            for fact2 in fact1..1000 {
                let product = fact1 * fact2;
                if is_palindrome(product, 10) && (product > largest_palindrome) {
                    largest_palindrome = product;
                }
            }
        }

        largest_palindrome.to_string()
    }
}
