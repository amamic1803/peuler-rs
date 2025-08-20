use crate::Solution;
use pmath::digits::{is_palindrome, reverse};

problem!(Problem0055, 55, "Lychrel Numbers");

impl Solution for Problem0055 {
    fn solve(&self) -> String {
        // index zero is not used
        // every other index is the number of iterations to become a palindrome for that number
        // if the number is not a palindrome after MAX_ITERATIONS, it is a Lychrel number
        // a vector of Option<u8> is used
        // None value means that the number has not been evaluated yet
        // Some(u8) value means that the number has been evaluated and the value is the number of iterations
        // since the limit of iterations is 50, the value can be stored in a u8
        // we will use the value of u8::MAX to represent a Lychrel number

        let mut iterations: Vec<Option<u8>> = vec![None; MAX_VALUE as usize];
        iterations[0] = Some(0); // set index zero to 0 so that it is not a Lychrel number

        // iterate through all numbers and find the number of iterations to become a palindrome
        // if the number is already analysed, skip it
        for n in 1..MAX_VALUE {
            if iterations[n as usize].is_none() {
                analyse_lychrel(n, 1, &mut iterations);
            }
        }

        // count the number of u8::MAX values (Lychrel numbers)
        iterations
            .into_iter()
            .filter(|&x| match x {
                None => false,
                Some(val) => val == u8::MAX,
            })
            .count()
            .to_string()
    }
}

const MAX_ITERATIONS: u8 = 50;
const MAX_VALUE: u128 = 10_000;

/// Analyse a number to find the number of iterations to become a palindrome
/// It is a recursive function with one of the arguments being a mutable reference to a vector
/// It mutates the vector itself so the solve() function doesn't need to use the return value
/// The return value is only used in recursive context
/// This is done this way to avoid recalculating the same number multiple times
/// For analysing small numbers, many bigger numbers smaller than MAX_VALUE are evaluated
/// Therefore, they don't need to be evaluated again
fn analyse_lychrel(n: u128, depth: u8, iterations: &mut [Option<u8>]) -> u8 {
    // first, it is checked whether the depth is greater than or equal to the maximum number of iterations
    // if it is the u8::MAX value is returned (this is a Lychrel number)
    if depth >= MAX_ITERATIONS {
        u8::MAX

    // then it is checked whether the number is smaller than MAX_VALUE and has already been evaluated
    // if it is, the value is returned
    } else if n < MAX_VALUE && iterations[n as usize].is_some() {
        iterations[n as usize].unwrap().saturating_add(1)

    // if none of these are the case, the number is evaluated
    } else {
        let next = n + reverse(n, 10); // next number (n + reverse(n))

        // in the next part before every return it is checked whether the number is smaller than MAX_VALUE
        // if it is, the result is also stored in the vector (besides being returned)

        // if the next number is a palindrome, the depth is returned
        if is_palindrome(next, 10) {
            if n < MAX_VALUE {
                iterations[n as usize] = Some(depth);
            }
            depth
        } else {
            // calculate the result of the recursive call
            let recursive_result = analyse_lychrel(next, depth + 1, iterations);

            // if the result is u8::MAX, the number is a Lychrel number
            if recursive_result == u8::MAX {
                if n < MAX_VALUE {
                    iterations[n as usize] = Some(u8::MAX);
                }
                u8::MAX

            // otherwise, the result is 1 + the recursive result
            } else {
                if n < MAX_VALUE {
                    iterations[n as usize] = Some(1 + recursive_result);
                }
                1 + recursive_result
            }
        }
    }
}
