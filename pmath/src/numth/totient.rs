use num_traits::{ConstOne, ConstZero, PrimInt};
use crate::numth::factor::distinct_prime_factors;

/// Euler's totient function.
///
/// It is defined as the number of positive integers less than `n` that are coprime to `n`.
/// # Arguments
/// * `n` - The integer to calculate the Euler's totient function of.
/// # Returns
/// * The Euler's totient function of the integer `n`.
/// # Panics
/// * If `n` is negative.
/// * If `n` cannot be converted to [f64].
/// # Example
/// ```
/// use pmath::phi;
///
/// assert_eq!(phi(0), 0);
/// assert_eq!(phi(1), 1);
/// assert_eq!(phi(2), 1);
/// assert_eq!(phi(3), 2);
/// assert_eq!(phi(4), 2);
/// assert_eq!(phi(5), 4);
/// ```
pub fn phi<T>(n: T) -> T
where
    T: PrimInt + ConstZero + ConstOne,
{
    distinct_prime_factors(n)
        .map(|(factor, _)| factor)
        .fold(n, |acc, factor| acc - (acc / factor))
}

/// Euler's totient function of integers from `0` to `n`.
/// # Arguments
/// * `n` - The integer up to which to calculate the Euler's totient function.
/// # Returns
/// * The Euler's totient function of integers from `0` to `n`.
///   Index represents the integer,
///   and the value at that index is the totient function of that integer.
/// # Panics
/// * If `n` cannot be converted to [usize].
/// # Example
/// ```
/// use pmath::phi_0_to_n;
///
/// assert_eq!(phi_0_to_n(5), vec![0, 1, 1, 2, 2, 4]);
/// ```
pub fn phi_0_to_n<T>(n: T) -> Vec<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    let n = n.to_usize().expect("Cannot convert n to usize.");
    let mut phi_values = Vec::with_capacity(n + 1);
    phi_values.push(T::ZERO);
    for _ in 0..n {
        phi_values.push(*phi_values.last().unwrap() + T::ONE);
    }

    for i in 2..=n {
        let ti = T::from(i).unwrap();
        if phi_values[i] == ti {
            for j in (i..=n).step_by(i) {
                phi_values[j] = phi_values[j] - phi_values[j] / ti;
            }
        }
    }

    phi_values
}
