//! **Problem 69** - *Totient Maximum*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(69, "Totient Maximum", solve)
}

use crate::shared::math::phi_1_to_n;
const MAX_N: u64 = 1_000_000; // maximum n value

fn solve() -> String {
    let phi_values = phi_1_to_n(MAX_N); // precompute phi values

    // find the maximum n/phi(n) ratio
    let mut max_ratio = 0.0;
    let mut max_index = 0;
    for n in 1..=MAX_N {
        let ratio = n as f64 / phi_values[n as usize] as f64;
        if ratio > max_ratio {
            max_ratio = ratio;
            max_index = n;
        }
    }

    // return the index (n) of the maximum ratio
    max_index.to_string()
}
