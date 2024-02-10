//! **Problem 64** - *Odd Period Square Roots*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        64,
        "Odd Period Square Roots",
        solve,
    )
}


use crate::shared::math::ContinuedFraction;
const MAX: i64 = 10_000;

fn solve() -> String {
    // number of continued fractions with odd period
    let mut odd_period = 0;

    for n in 2..=MAX {
        if let Some(periodic_part) = ContinuedFraction::from_sqrt(n).periodic() {
            if periodic_part.len() % 2 == 1 {
                odd_period += 1;
            }
        }
    }

    odd_period.to_string()
}
