//! **Problem 6** - *Sum Square Difference*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        6,
        "Sum Square Difference",
        solve,
    )
}


use crate::shared::numbers::{sum_n, sum_n_squares};

fn solve() -> String {
    let sum_of_squares = sum_n_squares(100);
    let square_of_sum = sum_n(100).pow(2);

    if square_of_sum > sum_of_squares {
        (square_of_sum - sum_of_squares).to_string()
    } else {
        (sum_of_squares - square_of_sum).to_string()
    }
}
