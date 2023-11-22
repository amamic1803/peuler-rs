//! **Problem 37** - *Truncatable Primes*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        37,
        "Truncatable Primes",
        solve,
    )
}


use itertools::Itertools;
use crate::shared::math::{digits, is_prime};

const TRUNC_PRIMES: u8 = 11;
const FIRST_DIGIT: [u8; 4] = [2, 3, 5, 7];
const MIDDLE_DIGIT: [u8; 4] = [1, 3, 7, 9];
const LAST_DIGIT: [u8; 2] = [3, 7];

fn solve() -> String {
    // first digit can only be 2, 3, 5, 7
    // it can't be 4, 6, 8, 9 because then it would be divisible by 2 (or 3)
    // it also can't be 1 because it is not a prime
    // last digit can only be 3, 7
    // it can't be 0, 2, 4, 5, 6, 8 because then the number would be divisible by 2 (or 5)
    // it also can't be 9 because then the number would be divisible by 3
    // and of course it can't be 1 because 1 is not a prime (when truncated)
    // middle digits can be 1, 3, 7, 9
    // they obviously can't be 0, 2, 4, 6, 8 because when truncated that would make the number divisible by 2
    // it also can't be 5 because when truncated that would make the number divisible by 5
    // now candidate numbers can be generated and checked if they are truncatable primes

    // vector to store truncatable primes
    let mut trunc_primes = Vec::with_capacity(TRUNC_PRIMES as usize);

    // start with 2 digit numbers and go up
    'outer: for n_len in 2_usize.. {

        // generate a vector of iterators for each digit
        // first element will be iterator over FIRST_DIGIT
        // middle elements (if any) will be iterator over MIDDLE_DIGIT
        // last element will be iterator over LAST_DIGIT
        let mut iterables = Vec::with_capacity(n_len);
        iterables.push(FIRST_DIGIT.iter().copied());
        for _ in 0..(n_len - 2) {
            iterables.push(MIDDLE_DIGIT.iter().copied());
        }
        iterables.push(LAST_DIGIT.iter().copied());

        // iterate over cartesian product of these iterators over digits (they make numbers)
        for i in iterables.into_iter().multi_cartesian_product() {
            let num = vec_to_int(&i);  // convert vector of digits to number

            // check if number is truncatable prime
            if is_trunc_prime(num) {
                trunc_primes.push(num);
                if trunc_primes.len() == TRUNC_PRIMES as usize {
                    break 'outer;
                }
            }
        }
    }

    // return sum of truncatable primes
    trunc_primes.iter().sum::<u64>().to_string()
}

/// Check if a number is a truncatable prime.
fn is_trunc_prime(x: u64) -> bool {
    if !is_prime(x).0 {
        return false;
    }
    for i in 1..digits(x).count() {
        if !is_prime(x % 10_u64.pow(i as u32)).0 ||
           !is_prime(x / 10_u64.pow(i as u32)).0
        { return false; }
    }
    true
}

/// Convert a vector of digits to an integer.
fn vec_to_int(n: &[u8]) -> u64 {
    let mut sum: u64 = 0;
    for digit in n {
        sum *= 10;
        sum += *digit as u64;
    }
    sum
}
