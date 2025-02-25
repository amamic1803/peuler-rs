//! **Problem 56** - *Powerful Digit Sum*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(56, "Powerful Digit Sum", solve)
}

use malachite::Natural;
use malachite::base::num::arithmetic::traits::Pow;
use malachite::base::num::conversion::traits::Digits;

fn solve() -> String {
    let mut max_sum: u32 = 0;
    for a in 1..100_u8 {
        for b in 1..100_u8 {
            let digits_sum: u32 = Natural::from(a).pow(b as u64).to_digits_desc(&10).iter().sum();
            if digits_sum > max_sum {
                max_sum = digits_sum;
            }
        }
    }
    max_sum.to_string()
}
