//! **Problem 50** - *Consecutive Prime Sum*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(50, "Consecutive Prime Sum", solve)
}

use crate::shared::math::sieve_of_eratosthenes;
use std::collections::HashSet;

const LIMIT: u64 = 1_000_000;

fn solve() -> String {
    let primes = sieve_of_eratosthenes(LIMIT - 1); // generate all primes less than 1_000_000
    let primes_set = primes.iter().copied().collect::<HashSet<_>>(); // store primes in a set for faster lookup
    let biggest_prime = primes[primes.len() - 1]; // get biggest prime from primes vector

    // we will use a sliding window to find the longest consecutive prime sum
    // we will start with the biggest window and shrink it by one and slide until we find a consecutive prime sum

    let mut sum: u64 = 0; // sum of consecutive primes
    let mut i: usize = 0; // first index of window
    let mut j: usize = 0; // first index after window
                          // create initial window (intentionally bigger than biggest_prime -> it is definitely not a consecutive prime sum)
    while sum <= biggest_prime {
        sum += primes[j];
        j += 1;
    }

    // flag that indicates where our window is located
    // true = window is on the left (from index 0 to something)
    // false = window is on the right
    let mut flag = true;

    // each iteration of this loop, window gets smaller by one
    'outer: loop {
        if flag {
            // window is on the left
            // we need to remove the last element (right)
            // and then slide the window to the right

            // remove last element
            sum -= primes[j - 1];
            j -= 1;

            // slide window to the right
            while sum < biggest_prime {
                if primes_set.contains(&sum) {
                    break 'outer;
                }

                sum += primes[j];
                j += 1;
                sum -= primes[i];
                i += 1;
            }

            // check if the last sum is a prime (not done in the loop above)
            if primes_set.contains(&sum) {
                break 'outer;
            }

            // set flag to false, as the window is now on the right
            flag = false;
        } else {
            // window is on the right
            // we need to remove the first element (left)
            // and then slide the window to the left

            // remove first element
            sum -= primes[i];
            i += 1;

            // slide window to the left
            while i > 0 {
                if primes_set.contains(&sum) {
                    break 'outer;
                }

                sum -= primes[j - 1];
                j -= 1;
                sum += primes[i - 1];
                i -= 1;
            }

            // check if the last sum is a prime (not done in the loop above)
            if primes_set.contains(&sum) {
                break 'outer;
            }

            // set flag to true, as the window is now on the left
            flag = true;
        }
    }

    sum.to_string()
}
