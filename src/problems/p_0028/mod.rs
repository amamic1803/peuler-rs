//! **Problem 28** - *Number Spiral Diagonals*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(28, "Number Spiral Diagonals", solve)
}

use crate::shared::math::{sum_n_odd, sum_n_odd_squares};

fn solve() -> String {
    // calculate top-right of every "layer" of spiral -> it is square of layer's side length -> calculate other corners by subtracting length of layer sides
    // formula for each "layer" simplifies to 4x^2 - 6x + 6
    // we can handle the first layer (1) as a special case
    // for x we can plug in numbers 3, 5, 7, 9, ... 1001
    // now the formula looks like this:
    // 1 + 4 * (3^2 + 5^2 + 7^2 + 9^2 + ... + 1001^2) - 6 * (3 + 5 + 7 + 9 + ... + 1001) + 6 * floor(1001 / 2)
    // or shorter:
    // 1 + 4 * (sum_of_squares_of_first_n_odd_numbers(floor(1001 / 2) + 1) - 1) - 6 * (sum_of_first_n_odd_numbers(floor(1001 / 2) + 1) - 1) + 6 * floor(1001 / 2)
    // we will be using predefined functions for sum of first n odd numbers and sum of squares of first n odd numbers, but if we expand them, we get:
    // 1 + 4 * ((floor(1001 / 2) + 1) * (2 * (floor(1001 / 2) + 1) + 1) * (2 * (floor(1001 / 2) + 1) - 1) / 3 - 1) - 6 * ((floor(1001 / 2) + 1)^2 - 1) + 6 * floor(1001 / 2)
    // note that these formulas are valid only for odd numbers, but that is ok because spiral size is always odd

    let result: u64 = 1 + 4 * (sum_n_odd_squares(SPIRAL_SIZE / 2 + 1) - 1) - 6 * (sum_n_odd(SPIRAL_SIZE / 2 + 1) - 1) + 6 * (SPIRAL_SIZE / 2);

    result.to_string()
}

// the size of the spiral (it is always odd)
const SPIRAL_SIZE: u64 = 1001;
