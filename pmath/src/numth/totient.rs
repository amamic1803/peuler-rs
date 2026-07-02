//! Totient function.

use crate::numth::factor::distinct_prime_factors;
use num_traits::{ConstOne, ConstZero, PrimInt};

/// Euler's totient function.
///
/// It is defined as the number of positive integers less than or equal to
/// `n` that are coprime to `n`.
/// # Arguments
/// * `n` - The integer to calculate the Euler's totient function of.
/// # Returns
/// * The Euler's totient function of the integer `n`.
/// # Panics
/// * If `n` is negative.
/// * If `n` cannot be converted to [f64].
/// # Example
/// ```
/// use pmath::numth::totient::phi;
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
/// use pmath::numth::totient::phi_0_to_n;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::numth::prime::coprime;

    // phi

    #[test]
    #[should_panic]
    fn phi_negative() {
        //! Test that [phi] panics when given negative input.
        phi(-1);
    }

    #[test]
    fn phi_primitive_types() {
        //! Test that [phi] works with primitive integer types.
        // unsigned types
        assert_eq!(phi(5u8), 4);
        assert_eq!(phi(5u16), 4);
        assert_eq!(phi(5u32), 4);
        assert_eq!(phi(5u64), 4);
        assert_eq!(phi(5u128), 4);
        assert_eq!(phi(5usize), 4);

        // signed types
        assert_eq!(phi(5i8), 4);
        assert_eq!(phi(5i16), 4);
        assert_eq!(phi(5i32), 4);
        assert_eq!(phi(5i64), 4);
        assert_eq!(phi(5i128), 4);
        assert_eq!(phi(5isize), 4);
    }

    #[test]
    fn phi_verify() {
        //! Verify the correctness of [phi].
        for i in 0..=100 {
            let mut count = 0;
            for j in 1..=i {
                if coprime(i, j) {
                    count += 1;
                }
            }
            assert_eq!(count, phi(i));
        }
    }

    // phi_0_to_n

    #[test]
    #[should_panic]
    fn phi_0_to_n_negative() {
        //! Test that [phi_0_to_n] panics when given negative input.
        phi_0_to_n(-1);
    }

    #[test]
    fn phi_0_to_n_primitive_types() {
        //! Test that [phi_0_to_n] works with primitive integer types.
        // unsigned types
        assert_eq!(phi_0_to_n(5u8), vec![0, 1, 1, 2, 2, 4]);
        assert_eq!(phi_0_to_n(5u16), vec![0, 1, 1, 2, 2, 4]);
        assert_eq!(phi_0_to_n(5u32), vec![0, 1, 1, 2, 2, 4]);
        assert_eq!(phi_0_to_n(5u64), vec![0, 1, 1, 2, 2, 4]);
        assert_eq!(phi_0_to_n(5u128), vec![0, 1, 1, 2, 2, 4]);
        assert_eq!(phi_0_to_n(5usize), vec![0, 1, 1, 2, 2, 4]);
        // signed types
        assert_eq!(phi_0_to_n(5i8), vec![0, 1, 1, 2, 2, 4]);
        assert_eq!(phi_0_to_n(5i16), vec![0, 1, 1, 2, 2, 4]);
        assert_eq!(phi_0_to_n(5i32), vec![0, 1, 1, 2, 2, 4]);
        assert_eq!(phi_0_to_n(5i64), vec![0, 1, 1, 2, 2, 4]);
        assert_eq!(phi_0_to_n(5i128), vec![0, 1, 1, 2, 2, 4]);
        assert_eq!(phi_0_to_n(5isize), vec![0, 1, 1, 2, 2, 4]);
    }

    #[test]
    fn phi_0_to_n_verify() {
        //! Verify the correctness of [phi_0_to_n].
        for i in 0..=20 {
            let phis = phi_0_to_n(i);
            for (j, val) in phis.into_iter().enumerate() {
                assert_eq!(phi(j), val);
            }
        }
    }
}
