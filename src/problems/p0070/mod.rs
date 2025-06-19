//! **Problem 70** - *Totient Permutation*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(70, "Totient Permutation", solve)
}

use crate::shared::math::{is_permutation, phi_1_to_n};
const MAX_N: u64 = 10_000_000; // maximum n value

fn solve() -> String {
    let phi_values = phi_1_to_n(MAX_N); // precompute phi values

    // find the minimum n/phi(n) ratio for (n, phi(n)) that are permutations of each other
    let mut min_ratio = f64::INFINITY;
    let mut min_index = 0;
    for n in 2..=MAX_N {
        if is_permutation(n, phi_values[n as usize], 10) {
            let ratio = n as f64 / phi_values[n as usize] as f64;
            if ratio < min_ratio {
                min_ratio = ratio;
                min_index = n;
            }
        }
    }

    // return the index (n) of the minimum ratio
    min_index.to_string()
}
