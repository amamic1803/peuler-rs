//! **Problem 38** - *Pandigital Multiples*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        38,
        "Pandigital Multiples",
        solve,
    )
}


use itertools::Itertools;
use crate::shared::math::{digits, slice_to_int};

fn solve() -> String {
    // to solve this one we will use this logic:
    // generate permutations of digits 1 to 9, but in reverse order
    // so that bigger numbers are generated first
    // that way when first number that satisfies the condition is found
    // it will be the biggest one
    // to find the multiplicand we will take first x digits of the permutation
    // and divide by 1 (which doesn't change the number)
    // for x we are taking 1, 2, 3, 4 because if we were to take 5 or more
    // then multiplicand * 1 and multiplicand * 2 would have a total of 10 digits
    // which is more than allowed
    // to check if chosen multiplicand satisfies the condition to produce a pandigital multiple
    // we generate the next product and check if it is equal to the next digits of the permutation

    for perm in (1..=9).rev().permutations(9) {
        for i in 1..=4 {
            let mut digit_count = i;
            let multiplicand = slice_to_int(&perm[0..i]);

            let mut pandigital_multiple = true;

            let mut n = 2;
            while digit_count < 9 {
                let next_product = multiplicand * n;
                n += 1;
                let next_digit_count = digit_count + digits(next_product).count();

                if (next_digit_count > 9) || (next_product != slice_to_int(&perm[digit_count..next_digit_count])) {
                    pandigital_multiple = false;
                    break;
                }

                digit_count = next_digit_count;
            }

            if pandigital_multiple {
                return slice_to_int(&perm).to_string();
            }
        }
    }

    String::from("No solution found.")
}
