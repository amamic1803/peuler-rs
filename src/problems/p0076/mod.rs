//! **Problem 76** - *Counting Summations*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(76, "Counting Summations", solve)
}

use crate::shared::math::partition_p;

fn solve() -> String {
    // the solution is the number of partitions of 100 minus 1
    // because 100 itself is counted as a partition

    (partition_p(100u8) - 1).to_string()
}
