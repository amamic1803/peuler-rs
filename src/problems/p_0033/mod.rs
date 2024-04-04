//! **Problem 33** - *Digit Cancelling Fractions*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(33, "Digit Cancelling Fractions", solve)
}

use crate::shared::math::{digits, gcd};

fn solve() -> String {
    let mut result: [u64; 2] = [1, 1];

    for numerator in 10..100 {
        for denominator in (numerator + 1)..100 {
            let digits_numerator: [u8; 2] = digits(numerator).collect::<Vec<u8>>().try_into().unwrap();
            let digits_denominator: [u8; 2] = digits(denominator).collect::<Vec<u8>>().try_into().unwrap();

            'outer: for m in 0..2 {
                for n in 0..2 {
                    if (digits_numerator[m] == digits_denominator[n]) && (digits_numerator[m] != 0) {
                        let lowest_terms = lowest_common_terms([digits_numerator[(m + 1) % 2] as u64, digits_denominator[(n + 1) % 2] as u64]);
                        if lowest_common_terms([numerator, denominator]) == lowest_terms {
                            result[0] *= lowest_terms[0];
                            result[1] *= lowest_terms[1];
                        }
                        break 'outer;
                    }
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
