//! Primes module

use crate::math::newtons_method;
use num_traits::{PrimInt, ToPrimitive, Unsigned};
use std::iter;

/// Inverse of the prime-counting function.
/// Estimates the number for which the prime-counting function is approximately n.
/// Uses the inverse of the prime number theorem.
/// Answer is approximated using Newton's method.
/// It is exact for n <= 3 and guaranteed to be an overestimate for n > 3.
/// # Arguments
/// * `n` - The number to estimate the inverse of the prime-counting function for.
/// # Returns
/// * `f64` - The estimated inverse of the prime-counting function for n.
/// # Example
/// ```
/// use peuler::math::prime::apcf;
/// assert_eq!(apcf(2), 3.0);
/// ```
pub fn apcf<T>(n: T) -> f64
where
    T: ToPrimitive,
{
    let n = n.to_f64().expect("Cannot convert to f64.");
    match n {
        0.0 => 0.0,
        1.0 => 2.0,
        2.0 => 3.0,
        3.0 => 5.0,
        _ => {
            let x0 = n + 1.0;
            let precision = 1e-10;
            let function = |x: f64| n * x.ln() - x;
            let derivative = |x: f64| n / x - 1.0;
            newtons_method(x0, precision, function, derivative)
        }
    }
}

/// Finds the distinct prime factors of a number and their powers.
/// The prime factors are returned in ascending order.
/// # Arguments
/// * `x` - The number to find the distinct prime factors of.
/// # Returns
/// * Iterator over the distinct prime factors and their powers. (prime_factor, power)
/// # Example
/// ```
/// use peuler::math::prime::distinct_prime_factors;
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

/// Checks if a number is prime.
/// # Arguments
/// * `num` - The number to check.
/// # Returns
/// * `bool` - Whether the number is prime.
/// * `u64` - The smallest divisor if the number is not prime, otherwise 1.
/// # Panics
/// If the number is less than 2.
/// # Example
/// ```
/// use peuler::math::prime::is_prime;
/// // 7 is prime
/// assert_eq!(is_prime(7), (true, 1));
/// // 12 is not prime, smallest divisor is 2
/// assert_eq!(is_prime(12), (false, 2));
/// ```
pub fn is_prime(n: u64) -> (bool, u64) {
    assert!(n >= 2, "Number must be greater than or equal to 2.");

    if n == 2 || n == 3 {
        (true, 1)
    } else if n % 2 == 0 {
        (false, 2)
    } else if n % 3 == 0 {
        (false, 3)
    } else {
        for i in (5..=((n as f64).sqrt().floor() as u64)).step_by(6) {
            if n % i == 0 {
                return (false, i);
            } else if n % (i + 2) == 0 {
                return (false, i + 2);
            }
        }

        (true, 1)
    }
}

/// Simple prime-counting function.
/// Estimates the number of primes less than or equal to x.
/// Uses the prime number theorem which states that the number of primes less than or equal to x is approximately x / ln(x).
/// It is exact for x <= 10 and guaranteed to be an underestimate for x > 10.
/// # Arguments
/// * `x` - The number to estimate the number of primes less than or equal to.
/// # Returns
/// * `f64` - The estimated number of primes less than or equal to x.
/// # Example
/// ```
/// use peuler::math::prime::pcf;
/// // number of primes less than or equal to 100: 25
/// assert!(pcf(100u8) < 25.0);
/// ```
pub fn pcf<T>(x: T) -> f64
where
    T: PrimInt + Unsigned + ToPrimitive,
{
    let x = x.to_f64().expect("Number too large.");
    match x {
        0.0..=1.0 => 0.0,
        2.0 => 1.0,
        3.0..=4.0 => 2.0,
        5.0..=6.0 => 3.0,
        7.0..=10.0 => 4.0,
        _ => x / x.ln(),
    }
}

/// Exact prime-counting function.
/// Finds the exact number of primes less than or equal to x.
/// Uses the sieve of Eratosthenes.
/// It is not recommended to use this function for large x.
/// # Arguments
/// * `x` - The number to find the number of primes less than or equal to.
/// # Returns
/// * `u64` - The exact number of primes less than or equal to x.
/// # Example
/// ```
/// use peuler::math::prime::pcf_exact;
/// // number of primes less than or equal to 100: 25
/// assert_eq!(pcf_exact(100), 25);
/// ```
pub fn pcf_exact(x: u64) -> u64 {
    sieve_of_eratosthenes(x).len() as u64
}

/// Returns the prime factors as an iterator.
/// If the factor appears multiple times, it will appear multiple times in the iterator.
/// Factors are returned in the ascending order.
/// # Arguments
/// * `x` - The number to find the prime factors of.
/// # Returns
/// * Iterator over the prime factors.
/// # Example
/// ```
/// use peuler::math::prime::prime_factors;
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

/// The sieve of Eratosthenes.
/// Finds all primes less than or equal to n.
/// # Arguments
/// * `n` - The number to find all primes less than or equal to.
/// # Returns
/// * `Vec<u64>` - All primes less than or equal to n.
/// # Example
/// ```
/// use peuler::math::prime::sieve_of_eratosthenes;
/// // primes less than or equal to 10: 2, 3, 5, 7
/// assert_eq!(sieve_of_eratosthenes(10), vec![2, 3, 5, 7]);
/// ```
pub fn sieve_of_eratosthenes(n: u64) -> Vec<u64> {
    match n {
        0..=1 => Vec::with_capacity(0),
        2 => vec![2],
        _ => {
            let mut results = vec![2];
            let mut sieve = vec![true; (n - 1) as usize / 2];

            let ind_to_val = |i: usize| ((i as u64) << 1) + 3; // calculate number value from index in sieve
            let val_to_ind = |v: u64| ((v - 3) >> 1) as usize; // calculate index in sieve from number value

            for prime_ind in 0..sieve.len() {
                if sieve[prime_ind] {
                    // get prime number value
                    let prime_val = ind_to_val(prime_ind);

                    // check all multiples of prime number value and mark them as not prime
                    // start checking at prime_val^2 (all smaller multiples have already been checked by smaller primes)
                    let mut check_val = prime_val * prime_val;
                    let mut check_ind = val_to_ind(check_val);
                    if check_ind >= sieve.len() {
                        break;
                    }

                    while check_ind < sieve.len() {
                        sieve[check_ind] = false;
                        // we want check_val to always be odd, prime_val is always odd, so we can just add prime_val * 2
                        // (because if we added 2 odd numbers we would get an even number)
                        check_val += prime_val << 1;
                        check_ind = val_to_ind(check_val);
                    }
                }
            }

            // convert sieve indices that are true to their corresponding number values and add them to results
            results.extend(
                sieve
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, prime)| if prime { Some(ind_to_val(i)) } else { None }),
            );

            // return results
            results
        }
    }
}
