//! **Problem 5** - *Smallest Multiple*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(5, "Smallest Multiple", solve)
}

use crate::shared::math::lcm_multiple;

fn solve() -> String {
    lcm_multiple(1u32..=20).to_string()
}
