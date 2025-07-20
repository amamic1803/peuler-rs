use std::iter;
use num_traits::{PrimInt, ToPrimitive, Unsigned};
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

/// Finds the number of divisors of a number.
/// # Arguments
/// * `n` - The number to find the number of divisors of.
/// # Returns
/// * `u64` - The number of divisors of the number.
/// # Example
/// ```
/// use peuler::math::factors::num_of_divisors;
/// // divisors of 12: 1, 2, 3, 4, 6, 12
/// assert_eq!(num_of_divisors(12), 6);
/// ```
pub fn num_of_divisors(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    // let n be a natural number
    // we can factorise n
    // n = p1^a1 * p2^a2 * p3^a3 * ... * pn^an
    // (where p1, p2, p3, ..., pn are prime numbers)
    // a function d(n) returns the number of divisors of n
    // obviously, d(n) = (a1 + 1) * (a2 + 1) * (a3 + 1) * ... * (an + 1)

    distinct_prime_factors(n).map(|(_, a)| a + 1).product()
}

/// Finds the number of divisors of numbers from 1 to n.
/// # Arguments
/// * `n` - The number to find the number of divisors of.
/// # Returns
/// * `Vec<u64>` - The number of divisors of numbers from 0 to n. Index represents the number.
/// # Example
/// ```
/// use peuler::math::factors::num_of_divisors_1_to_n;
/// assert_eq!(num_of_divisors_1_to_n(10u8), vec![0, 1, 2, 2, 3, 2, 4, 2, 4, 3, 4]);
/// ```
pub fn num_of_divisors_1_to_n<T>(n: T) -> Vec<u64>
where
    T: PrimInt + Unsigned + ToPrimitive,
{
    let n = n.to_usize().expect("Number too large.");
    let mut divisors = vec![1; n + 1];
    divisors[0] = 0;
    for i in 2..=n {
        for j in (i..=n).step_by(i) {
            divisors[j] += 1;
        }
    }
    divisors
}

/// Finds the number of proper divisors of a number.
/// Proper divisors are all divisors of a number except the number itself.
/// # Arguments
/// * `n` - The number to find the number of proper divisors of.
/// # Returns
/// * `u64` - The number of proper divisors of the number.
/// # Example
/// ```
/// use peuler::math::factors::num_of_proper_divisors;
/// // proper divisors of 12: 1, 2, 3, 4, 6
/// assert_eq!(num_of_proper_divisors(12), 5);
/// ```
pub fn num_of_proper_divisors(n: u64) -> u64 {
    num_of_divisors(n).saturating_sub(1)
}

/// Finds the number of proper divisors of numbers from 1 to n.
/// Proper divisors are all divisors of a number except the number itself.
/// # Arguments
/// * `n` - The number to find the number of proper divisors of.
/// # Returns
/// * `Vec<u64>` - The number of proper divisors of numbers from 0 to n. Index represents the number.
/// # Example
/// ```
/// use peuler::math::factors::num_of_proper_divisors_1_to_n;
/// assert_eq!(num_of_proper_divisors_1_to_n(10u8), vec![0, 0, 1, 1, 2, 1, 3, 1, 3, 2, 3]);
/// ```
pub fn num_of_proper_divisors_1_to_n<T>(n: T) -> Vec<u64>
where
    T: PrimInt + Unsigned + ToPrimitive,
{
    let n = n.to_usize().expect("Number too large.");
    let mut divisors = vec![0; n + 1];
    for i in 2..=n {
        for j in (i..=n).step_by(i) {
            divisors[j] += 1;
        }
    }
    divisors
}

/// Finds the sum of the divisors of a number.
/// # Arguments
/// * `n` - The number to find the sum of the divisors of.
/// # Returns
/// * `u64` - The sum of the divisors of the number.
/// # Example
/// ```
/// use peuler::math::factors::sum_of_divisors;
/// // sum of divisors of 10 is 18
/// assert_eq!(sum_of_divisors(10), 18);
/// ```
pub fn sum_of_divisors(n: u64) -> u64 {
    // let σ(n) be the sum of the divisors of n
    // let p be a prime number
    // then σ(p) = p + 1
    // and σ(p^a) = 1 + p + p^2 + ... + p^a = Σ(k=0, a)p^k = (p^(a + 1) - 1) / (p - 1)
    // we can see that
    // σ(p1^a * p2^b) = 1 + p1 + p1^2 + ... + p1^a + p1*p2 + p1^2*p2 + ... + p1^a*p2 + p1*p2^2 + ... + p1^a*p2^2 + ... + p1^a*p2^b
    // = Σ(k=0, a)p1^k * Σ(k=0, b)p2^k = σ(p1^a) * σ(p2^b)
    // = (p1^(a + 1) - 1) / (p1 - 1) * (p2^(b + 1) - 1) / (p2 - 1)
    // we can also increase this to more than 2 prime factors

    // first we check if n is 0, if it is then we return 0
    if n == 0 {
        0
    } else {
        // if n is not zero then we proceed to find the sum of the divisors
        // note that σ(1) = 1

        // we find the prime factors of n
        // for each we calculate the sum of the divisors of that prime factor
        // and multiply them together

        distinct_prime_factors(n)
            .map(|(p, a)| (p.pow(a as u32 + 1) - 1) / (p - 1))
            .product()
    }
}

/// Finds the sum of the divisors of numbers from 1 to n.
/// # Arguments
/// * `n` - The number to find the sum of the divisors of.
/// # Returns
/// * `Vec<u64>` - The sum of the divisors of numbers from 0 to n. Index represents the number.
/// # Example
/// ```
/// use peuler::math::factors::sum_of_divisors_1_to_n;
/// // sum of divisors of 1 is 1
/// assert_eq!(sum_of_divisors_1_to_n(1), vec![0, 1]);
/// // sum of divisors of 2 is 3
/// assert_eq!(sum_of_divisors_1_to_n(2), vec![0, 1, 3]);
/// // sum of divisors of 10 is 18
/// assert_eq!(sum_of_divisors_1_to_n(10), vec![0, 1, 3, 4, 7, 6, 12, 8, 15, 13, 18]);
/// ```
pub fn sum_of_divisors_1_to_n(n: u64) -> Vec<u64> {
    let mut divisors = vec![0; (n + 1) as usize];
    for i in 1..=n {
        for j in (i..=n).step_by(i as usize) {
            divisors[j as usize] += i;
        }
    }
    divisors
}

/// Finds the sum of the proper divisors of a number.
/// Proper divisors are all divisors of a number except the number itself.
/// # Arguments
/// * `n` - The number to find the sum of the proper divisors of.
/// # Returns
/// * `u64` - The sum of the proper divisors of the number.
/// # Example
/// ```
/// use peuler::math::factors::sum_of_proper_divisors;
/// // sum of proper divisors of 10 is 8
/// assert_eq!(sum_of_proper_divisors(10), 8);
/// ```
pub fn sum_of_proper_divisors(n: u64) -> u64 {
    // sum of proper divisors is equal to the sum of all divisors minus the number itself
    sum_of_divisors(n) - n
}

/// Finds the sum of the proper divisors of numbers from 1 to n.
/// Proper divisors are all divisors of a number except the number itself.
/// # Arguments
/// * `n` - The number to find the sum of the proper divisors of.
/// # Returns
/// * `Vec<u64>` - The sum of the proper divisors of numbers from 0 to n. Index represents the number.
/// # Example
/// ```
/// use peuler::math::factors::sum_of_proper_divisors_1_to_n;
/// // sum of proper divisors of 1 is 0
/// assert_eq!(sum_of_proper_divisors_1_to_n(1), vec![0, 0]);
/// // sum of proper divisors of 2 is 1
/// assert_eq!(sum_of_proper_divisors_1_to_n(2), vec![0, 0, 1]);
/// // sum of proper divisors of 10 is 8
/// assert_eq!(sum_of_proper_divisors_1_to_n(10), vec![0, 0, 1, 1, 3, 1, 6, 1, 7, 4, 8]);
/// ```
pub fn sum_of_proper_divisors_1_to_n(n: u64) -> Vec<u64> {
    let mut divisors = vec![0; (n + 1) as usize];
    for i in 1..=n {
        for j in ((2 * i)..=n).step_by(i as usize) {
            divisors[j as usize] += i;
        }
    }
    divisors
}
