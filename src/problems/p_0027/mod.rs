//! **Problem 27** - *Quadratic Primes*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(27, "Quadratic Primes", solve)
}

use crate::shared::math::{is_prime, sieve_of_eratosthenes};

fn solve() -> String {
    // because n starts from 0, b must be prime
    // so b must only be checked among primes <= 1000
    // for n = 1, 1 + a + b must be prime
    // so a can be calculated as a = some_prime - b - 1
    // since a can be negative, some_prime can be smaller than b
    // in fact some_prime can be as low as 2
    // the upper limit on some_prime is 2000
    // that is because max value of b is 2000 (inclusive), max value of a is 999 and there is also a +1
    // let's first generate a list of primes <= 2000
    let primes_list = sieve_of_eratosthenes(2000);

    // variable to store the maximum number of consecutive primes
    let mut max_consecutive = 0;
    // variable to store the product of a and b that produces the maximum number of consecutive primes
    let mut max_product = 0;

    // be is chosen from primes_list
    // since primes are generated in ascending order,
    // when primes_list[b_ind] > 1000, there is no need to check further
    for b in &primes_list {
        let b = *b as i64;
        if b > 1000 {
            break;
        }

        // for every prime from primes_list calculate a and check the number of consecutive primes
        // if the a is greater than 1000, there is no need to check further
        // because primes are generated in ascending order, next a will be even greater
        for prime in &primes_list {
            let a = *prime as i64 - b - 1;
            if a >= 1000 {
                break;
            }

            // find the number of consecutive primes
            let mut consecutive_primes = 0;
            for n in 0.. {
                let polynomial = (n * n) + (a * n) + b;
                if polynomial < 2 {
                    break;
                }
                if is_prime(polynomial as u64).0 {
                    consecutive_primes += 1;
                } else {
                    break;
                }
            }

            // if the number of consecutive primes is greater than the previous maximum, update the maximum
            // and store the product of a and b
            if consecutive_primes > max_consecutive {
                max_consecutive = consecutive_primes;
                max_product = a * b;
            }
        }
    }

    // return the product of a and b that produces the maximum number of consecutive primes
    max_product.to_string()
}
