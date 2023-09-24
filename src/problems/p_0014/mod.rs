//! **Problem 14** - *Longest Collatz Sequence*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        14,
        "Longest Collatz Sequence",
        solve,
    )
}

use crate::shared::math::collatz_seq;

fn solve() -> String {
    (1..1_000_000)
        .map(|n| (n, collatz_seq(n).count()))
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .0
        .to_string()
}
