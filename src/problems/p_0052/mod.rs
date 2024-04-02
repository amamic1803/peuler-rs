//! **Problem 52** - *Permuted Multiples*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(52, "Permuted Multiples", solve)
}

use crate::shared::math::digits;

fn solve() -> String {
    // temp vectors for storing digits
    let mut digits1 = Vec::new();
    let mut digits2 = Vec::new();

    // n = number of digits
    for n in 1.. {
        // start = smallest number with n digits
        let start = 10_u64.pow(n - 1);
        // end = largest number with n digits (divided by 6 because we are looking for a number that is a multiple of 6)
        let end = (10_u64.pow(n) - 1) / 6;

        // check all numbers in that range
        for num in start..=end {
            // store and sort digits of num in digits1
            digits1.clear();
            for digit in digits(num) {
                digits1.push(digit);
            }
            digits1.sort();

            // check multiples of num
            for multiple in 2..=6 {
                // store and sort digits of multiple in digits2
                digits2.clear();
                for digit in digits(num * multiple) {
                    digits2.push(digit);
                }
                digits2.sort();

                // if digits don't match, condition is not met, so break and move to next num
                if digits1 != digits2 {
                    break;
                }

                // if the previous if didn't break out and the multiple is 6, then we found the number we are looking for (because none of the previous multiples broke out)
                if multiple == 6 {
                    return num.to_string();
                }
            }
        }
    }

    unreachable!("Previous loop can only be exited by returning a value");
}
