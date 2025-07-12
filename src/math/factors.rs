use std::iter;
use crate::math::primes::sieve_of_eratosthenes;

/// Returns the prime factors as an iterator.
/// If the factor appears multiple times, it will appear multiple times in the iterator.
/// Factors are returned in the ascending order.
/// # Arguments
/// * `x` - The number to find the prime factors of.
/// # Returns
/// * Iterator over the prime factors.
/// # Example
/// ```
/// use peuler::math::factors::prime_factors;
/// // prime factors of 12: 2, 2, 3
/// assert_eq!(prime_factors(12).collect::<Vec<_>>(), vec![2, 2, 3]);
/// ```
pub fn prime_factors(mut x: u64) -> impl Iterator<Item = u64> {
    // calculate primes that are less than or equal to the square root of x
    let mut prime_table = sieve_of_eratosthenes((x as f64).sqrt().floor() as u64).into_iter();

    // check those primes
    let mut factor = prime_table.next().unwrap_or(2);
    iter::from_fn(move || {
        while x != 0 {
            if x % factor == 0 {
                x /= factor;
                return Some(factor);
            }
            match prime_table.next() {
                Some(next_factor) => {
                    factor = next_factor;
                }
                // if there are no more primes x is either 1 or a prime number (last factor)
                None => {
                    if x != 1 {
                        factor = x;
                    } else {
                        return None;
                    }
                }
            }
        }
        None
    })
}

/// Finds the distinct prime factors of a number and their powers.
/// The prime factors are returned in ascending order.
/// # Arguments
/// * `x` - The number to find the distinct prime factors of.
/// # Returns
/// * Iterator over the distinct prime factors and their powers. (prime_factor, power)
/// # Example
/// ```
/// use peuler::math::factors::distinct_prime_factors;
/// // distinct prime factors of 12: (2, 2), (3, 1)
/// assert_eq!(distinct_prime_factors(12).collect::<Vec<_>>(), vec![(2, 2), (3, 1)]);
/// // distinct prime factors of 2048: (2, 11)
/// assert_eq!(distinct_prime_factors(2048).collect::<Vec<_>>(), vec![(2, 11)]);
/// // distinct prime factors of 134043: (3, 1), (7, 1), (13, 1), (491, 1)
/// assert_eq!(distinct_prime_factors(134043).collect::<Vec<_>>(), vec![(3, 1), (7, 1), (13, 1), (491, 1)]);
/// ```
pub fn distinct_prime_factors(x: u64) -> impl Iterator<Item = (u64, u64)> {
    let mut prime_factors = prime_factors(x);
    let mut factor = 0;
    let mut power = 0;
    iter::from_fn(move || {
        loop {
            match prime_factors.next() {
                Some(next_factor) => {
                    if next_factor == factor {
                        power += 1;
                    } else {
                        let ret = (factor, power);
                        factor = next_factor;
                        power = 1;
                        if ret.1 != 0 {
                            return Some(ret);
                        }
                    }
                }
                None => {
                    return if power != 0 {
                        let ret = (factor, power);
                        power = 0;
                        Some(ret)
                    } else {
                        None
                    };
                }
            }
        }
    })
}

