//! Functions related to prime numbers.

use crate::math::newtons_method;
use num_traits::{ConstOne, ConstZero, PrimInt, ToPrimitive};

#[cfg_attr(doc, katexit::katexit)]
/// A prime-counting function.
///
/// Estimates the number of primes less than or equal to $x$.
/// Uses the prime number theorem which states that the number of primes
/// less than or equal to $x$ is approximately $x / \\ln{(x)}$.
/// It is exact for $x < 11$ and guaranteed to be an underestimate for $x >= 11$.
/// # Arguments
/// * `x` - The number to estimate the number of primes less than or equal to.
/// # Returns
/// * `f64` - The estimated number of primes less than or equal to $x$.
/// # Panics
/// If the number is negative or too large to convert to `f64`.
/// # Example
/// ```
/// use peuler::math::primes::pcf;
///
/// // 2, 3, 5, 7
/// assert_eq!(pcf(7), 4.0);
///
/// // number of primes less than or equal to 100: 25
/// assert!(pcf(100u8) < 25.0);
/// ```
pub fn pcf<T>(x: T) -> f64
where
    T: ToPrimitive,
{
    let x = x.to_f64().expect("Number too large.");
    if x < 0.0 {
        panic!("Number must be non-negative.");
    } else if x < 2.0 {
        0.0
    } else if x < 3.0 {
        1.0
    } else if x < 5.0 {
        2.0
    } else if x < 7.0 {
        3.0
    } else if x < 11.0 {
        4.0
    } else {
        x / x.ln()
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// An inverse prime-counting function.
///
/// Estimates the number for which the prime-counting function is approximately $n$.
/// Uses the inverse of the prime number theorem.
/// The answer is approximated using Newton's method.
/// For $n <= 3$ it is calculated exactly using $\\lfloor n \\rfloor$ and
/// guaranteed to be an overestimate for $n > 3$.
/// # Arguments
/// * `n` - The number to estimate the inverse of the prime-counting function for.
/// # Returns
/// * `f64` - The estimated inverse of the prime-counting function for $n$.
/// # Panics
/// If the number is negative or cannot be converted to `f64`.
/// # Example
/// ```
/// use peuler::math::primes::apcf;
///
/// assert_eq!(apcf(2), 3.0);
/// assert!(apcf(25) > 100.0);
/// ```
pub fn apcf<T>(n: T) -> f64
where
    T: ToPrimitive,
{
    let mut n = n.to_f64().expect("Cannot convert to f64.");
    if n < 0.0 {
        panic!("Number must be non-negative.");
    } else if n <= 3.0 {
        n = n.floor();
        match n {
            0.0 => 0.0,
            1.0 => 2.0,
            2.0 => 3.0,
            3.0 => 5.0,
            _ => unreachable!()
        }
    } else {
        let x0 = n + 1.0;
        let precision = 1e-10;
        let function = |x: f64| n * x.ln() - x;
        let derivative = |x: f64| n / x - 1.0;
        newtons_method(x0, precision, function, derivative)
    }
}

/// Checks if a number is prime.
/// # Arguments
/// * `n` - The number to check the primality of.
/// # Returns
/// * `bool` - Whether the number is prime.
/// * `T` - The smallest divisor if the number is not prime, otherwise `1`.
/// # Panics
/// If the number is less than `2` or cannot be converted to `f64`.
/// # Example
/// ```
/// use peuler::math::primes::is_prime;
/// // 7 is prime
/// assert_eq!(is_prime(7), (true, 1));
/// // 12 is not prime, the smallest divisor is 2
/// assert_eq!(is_prime(12), (false, 2));
/// ```
pub fn is_prime<T>(n: T) -> (bool, T)
where
    T: PrimInt + ConstZero + ConstOne
{
    let t2 = T::from(2).unwrap();
    assert!(n >= t2, "Number must be greater than or equal to 2.");
    let t3 = T::from(3).unwrap();
    let t6 = T::from(6).unwrap();

    if n == t2 || n == t3 {
        (true, T::ONE)
    } else if n % t2 == T::ZERO {
        (false, t2)
    } else if n % t3 == T::ZERO {
        (false, t3)
    } else {
        let upper_bound = T::from(n.to_f64().expect("Cannot convert to f64.").sqrt().floor()).unwrap();
        let mut i = T::from(5).unwrap();
        while i <= upper_bound {
            if n % i == T::ZERO {
                return (false, i);
            } else if n % (i + t2) == T::ZERO {
                return (false, i + t2);
            }
            i = i + t6; // increment by 6
        }

        (true, T::ONE)
    }
}

/// The sieve of Eratosthenes.
///
/// Finds all primes less than or equal to `n`.
/// # Arguments
/// * `n` - The number to find all primes less than or equal to.
/// # Returns
/// * `Vec<T>` - All primes less than or equal to `n`.
/// # Panics
/// If the sieve requires more elements than can be represented as `usize`.
/// # Example
/// ```
/// use peuler::math::primes::sieve_of_eratosthenes;
///
/// // primes less than or equal to 10: 2, 3, 5, 7
/// assert_eq!(sieve_of_eratosthenes(10), vec![2, 3, 5, 7]);
/// ```
pub fn sieve_of_eratosthenes<T>(n: T) -> Vec<T>
where
    T: PrimInt + ConstOne
{
    let t2 = T::from(2).unwrap();
    if n < t2 {
        Vec::with_capacity(0)
    } else if n == t2 {
        vec![t2]
    } else {
        let mut results = vec![t2];
        let mut sieve = vec![true; ((n - T::ONE) / t2).to_usize().expect("Sieve too large.")];

        let ind_to_val = |i: usize| ((i as u64) << 1) + 3; // calculate number value from index in sieve
        let val_to_ind = |v: u64| ((v - 3) >> 1) as usize; // calculate index in sieve from number value

        for prime_ind in 0..sieve.len() {
            if sieve[prime_ind] {
                // get prime number value
                let prime_val = ind_to_val(prime_ind);
                let prime_val_2 = prime_val << 1; // prime_val * 2, used to skip even numbers

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
                    check_val += prime_val_2;
                    check_ind = val_to_ind(check_val);
                }
            }
        }

        // convert sieve indices that are true to their corresponding number values and add them to results
        results.extend(
            sieve
                .into_iter()
                .enumerate()
                .filter_map(|(i, prime)| if prime { Some(T::from(ind_to_val(i)).unwrap()) } else { None }),
        );

        // return results
        results
    }
}
