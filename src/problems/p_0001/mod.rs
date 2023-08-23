//! **Problem 1** - *Multiples of 3 or 5*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        1,
        "Multiples of 3 or 5",
        solve,
    )
}

fn solve() -> String {
    let mut sum: usize = 0;
    for i in 1..1000 {
        if (i % 3 == 0) || (i % 5 == 0) {
            sum += i;
        }
    }
    sum.to_string()
}