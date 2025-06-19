//! **Problem 72** - *Counting Fractions*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(72, "Counting Fractions", solve)
}

use crate::shared::math::phi_1_to_n;

fn solve() -> String {
    // this is Farey sequence
    // the number of elements in the Farey sequence F(n) is given as:
    // F(n) = 1 + Σ(φ(i)) for i = 1 to n
    let farey_elements = 1 + phi_1_to_n(1000000).into_iter().sum::<u64>();

    // the problem excludes 0 and 1, so subtract 2 and return result
    (farey_elements - 2).to_string()
}
