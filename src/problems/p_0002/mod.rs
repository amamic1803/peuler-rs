//! **Problem 2** - *Even Fibonacci Numbers*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        2,
        "Even Fibonacci Numbers",
        solve,
    )
}

fn solve() -> String {
    let mut sum: usize = 0;
    let mut values = [1, 2];
    while values[1] < 4000000 {
        if values[1] % 2 == 0 {
            sum += values[1];
        }
        (values[0], values[1]) = (values[1], values[0] + values[1]);
    }
    sum.to_string()
}