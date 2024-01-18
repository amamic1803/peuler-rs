//! **Problem 77** - *Prime Summations*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        77,
        "Prime Summations",
        solve,
    )
}


use crate::shared::math::partition_prime;


fn solve() -> String {
    for i in 0.. {
        if partition_prime(i) > 5000 {
            return i.to_string();
        }
    }
    unreachable!()
}
