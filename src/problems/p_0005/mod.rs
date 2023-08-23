//! **Problem 5** - *Smallest Multiple*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        5,
        "Smallest Multiple",
        solve,
    )
}


use crate::shared::numbers::lcm_multiple;

fn solve() -> String {
    let mut divisors: [u128; 20] = [0; 20];
    for (i, position) in divisors.iter_mut().enumerate() {
        *position = (i + 1) as u128;
    }
    lcm_multiple(&divisors).to_string()
}
