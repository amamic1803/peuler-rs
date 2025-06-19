//! **Problem 33** - *Digit Cancelling Fractions*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(33, "Digit Cancelling Fractions", solve)
}

use crate::shared::math::{digits, gcd};
use itertools::Itertools;

fn solve() -> String {
    let mut result = [1, 1];

    for numerator in 10..100 {
        for denominator in (numerator + 1)..100 {
            let mut digits_numerator_iter = digits(numerator, 10);
            let mut digits_denominator_iter = digits(denominator, 10);
            let digits_numerator = [
                digits_numerator_iter.next().unwrap(),
                digits_numerator_iter.next().unwrap(),
            ];
            let digits_denominator = [
                digits_denominator_iter.next().unwrap(),
                digits_denominator_iter.next().unwrap(),
            ];

            for (m, n) in (0..2).cartesian_product(0..2) {
                if (digits_numerator[m] == digits_denominator[n]) && (digits_numerator[m] != 0) {
                    let lowest_terms = lowest_common_terms([
                        digits_numerator[(m + 1) % 2] as u64,
                        digits_denominator[(n + 1) % 2] as u64,
                    ]);
                    if lowest_common_terms([numerator, denominator]) == lowest_terms {
                        result[0] *= lowest_terms[0];
                        result[1] *= lowest_terms[1];
                    }
                    break;
                }
            }
        }
    }

    result = lowest_common_terms(result);
    result[1].to_string()
}

fn lowest_common_terms(fraction: [u64; 2]) -> [u64; 2] {
    let div = gcd(fraction[0], fraction[1]);
    [fraction[0] / div, fraction[1] / div]
}
