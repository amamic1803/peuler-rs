//! **Problem 57** - *Square Root Convergents*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        57,
        "Square Root Convergents",
        solve,
    )
}


use malachite::{Natural, Rational};
use malachite::num::arithmetic::traits::FloorLogBase;
use malachite::num::basic::traits::{One, Two};

fn solve() -> String {
    let ten_rational = Natural::from(10_u8);
    let mut infinite_fraction = Rational::from_unsigneds(0_u8, 1_u8);
    let mut bigger_numerators = 0;

    for _ in 0..1000 {
        infinite_fraction = Rational::ONE / (Rational::TWO + &infinite_fraction);

        let result = Rational::ONE + &infinite_fraction;

        let numerator_digits = result.numerator_ref().floor_log_base(&ten_rational) + 1;
        let denominator_digits = result.denominator_ref().floor_log_base(&ten_rational) + 1;

        if numerator_digits > denominator_digits {
            bigger_numerators += 1;
        }
    }

    bigger_numerators.to_string()
}
