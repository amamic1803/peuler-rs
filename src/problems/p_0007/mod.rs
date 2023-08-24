//! **Problem 7** - *10001st Prime*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        7,
        "10001st Prime",
        solve,
    )
}


use crate::shared::math::{apcf, sieve_of_eratosthenes};

fn solve() -> String {
    sieve_of_eratosthenes(apcf(10001).round() as u64)[10_000].to_string()
}
