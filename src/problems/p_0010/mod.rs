//! **Problem 10** - *Summation of Primes*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        10,
        "Summation of Primes",
        solve,
    )
}


use crate::shared::math::sieve_of_eratosthenes;

fn solve() -> String {
    sieve_of_eratosthenes(1_999_999).iter().sum::<u64>().to_string()
}
