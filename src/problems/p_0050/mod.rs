//! **Problem 50** - *Consecutive Prime Sum*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        50,
        "Consecutive Prime Sum",
        solve,
    )
}

// TODO: optimize

use crate::shared::math::sieve_of_eratosthenes;

fn solve() -> String {
    let primes = sieve_of_eratosthenes(1_000_000 - 1);  // generate all primes less than 1_000_000

    let mut max_consec_primes = 0;  // maximum number of consecutive primes found to sum to a prime
    let mut max_num = 0;  // the prime that has the maximum number of consecutive primes that sum to it

    // each iteration we check one prime number
    for i in &primes {
        // we start by summing the primes from the start until we would be over the prime we are checking
        // so end with the sum that is lower than the prime we are checking
        // the we add the next number to the sum (so the sum surpasses the prime we are checking)
        // and then we keep removing the prime numbers from the start from sum (until the sum is less than the prime we are checking)
        // if manage to get the sum to equal the prime, then we check if the number of primes we added is greater than the current max
        // if we don't manage to get the sum to equal the prime, we just continue to the next prime

        let mut sum = 0;
        let mut curr_ind = 0;

        // sum primes while sum is less than prime we are checking
        while sum + primes[curr_ind] < *i {
            sum += primes[curr_ind];
            curr_ind += 1;
        }

        // if we managed to get the sum to equal the prime we are checking, then check if the number of primes we added is greater than the current max
        if sum == *i {
            if max_consec_primes < curr_ind {
                max_consec_primes = curr_ind;
                max_num = *i;
            }
        } else {
            // we add the next prime in the sequence to the sum
            // now the sum is greater than the prime we are checking for
            // then we subtract primes from the start of the sequence until the sum is less than the prime we are checking for
            // we stop if we get the sum to equal the prime we are checking for
            // or if we get the min and max indexes to match

            let mut curr_min_ind = 0;
            while (sum != *i) && (curr_min_ind != curr_ind) {
                sum += primes[curr_ind];
                curr_ind += 1;
                while sum > *i {
                    sum -= primes[curr_min_ind];
                    curr_min_ind += 1;
                }
            }

            // now we calculate the number of consecutive primes that sum to the prime we are checking for
            // it is not necessary to check if the sum is equal to the prime we are checking for
            // because by the time we get here, the sum is equal to the prime we are checking for (*)
            // (*) the sum might not be equal to the prime we are checking for if we get the min and max indexes to match,
            // but in that case the number of consecutive primes that sum to the prime we are checking for is 0 so it won't change the max
            if max_consec_primes < (curr_ind - curr_min_ind) {
                max_consec_primes = curr_ind - curr_min_ind;
                max_num = *i;
            }
        }
    }

    max_num.to_string()
}
