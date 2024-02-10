//! **Problem 66** - *Diophantine Equation*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        66,
        "Diophantine Equation",
        solve,
    )
}


use malachite::Natural;
use malachite::num::basic::traits::{Zero, One};
use crate::shared::math::ContinuedFraction;

const MAX: u16 = 1000;

fn solve() -> String {
    let mut max_x = Natural::ZERO;
    let mut max_d = 0;
    for d in 1..MAX {
        let sqrt_continued_fraction = ContinuedFraction::from_sqrt(d as i64);
        for convergent in sqrt_continued_fraction.convergents() {
            if convergent.denominator_ref() * convergent.denominator_ref() * Natural::from(d) + Natural::ONE == convergent.numerator_ref() * convergent.numerator_ref() {
                if convergent.numerator_ref() > &max_x {
                    max_x = convergent.numerator_ref().clone();
                    max_d = d;
                }
                break;
            }
        }
    }

    max_d.to_string()
}
