//! **Problem 9** - *Special Pythagorean Triplet*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(9, "Special Pythagorean Triplet", solve)
}

fn solve() -> String {
    let limit = 1000;

    for a in 1..(limit / 3 + 1) {
        for b in a..((limit - a) / 2 + 1) {
            let c = limit - a - b;
            if a * a + b * b == c * c {
                return (a * b * c).to_string();
            }
        }
    }

    "No solution found!".to_string()
}
