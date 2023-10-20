//! **Problem 65** - *Convergents of e*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        65,
        "Convergents of e",
        solve,
    )
}


use malachite::Rational;
use malachite::num::basic::traits::{One, Two};
use malachite::num::conversion::traits::Digits;

fn solve() -> String {
    let mut fraction = Rational::from_unsigneds(0_u8, 1_u8);

    // uses 99 here since the first iteration is done outside the loop (the fraction is initialized to 0/1)
    for i in (0..99).rev() {
        let fraction_sum_part = if i % 3 == 1 { Rational::TWO * Rational::from(i / 3 + 1) } else { Rational::ONE };
        fraction = Rational::ONE / (fraction_sum_part + &fraction);
    }

    let result = Rational::TWO + &fraction;

    result.numerator_ref().to_digits_desc(&10_u32).iter().sum::<u32>().to_string()
}
