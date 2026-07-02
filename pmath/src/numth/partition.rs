//! Partition functions.

use crate::numth::prime::sieve_of_eratosthenes;
use num_traits::{ConstOne, ConstZero, PrimInt};

#[cfg_attr(doc, katexit::katexit)]
/// Partition function.
///
/// Partition function $p$ is defined as the number of ways an integer
/// can be written as a sum of positive integers.
/// These sums are called partitions of the integer.
/// It is calculated using the recurrence relation:
/// $$
///     p(n) =
///     \\begin{cases}
///         0 & \\text{if}\\quad n < 0 \\\\
///         1 & \\text{if}\\quad n = 0 \\\\
///         \\sum\_{k=1}\^n (-1)\^{k+1} \\cdot (p(n - \\frac{k \\cdot (3k - 1)}{2}) + p(n - \\frac{k \\cdot (3k + 1)}{2})) & \\text{if}\\quad n > 0
///     \\end{cases}
/// $$
/// # Arguments
/// * `n` - The integer to find the number of partitions of.
/// # Returns
/// * The number of partitions of the integer.
/// # Panics
/// * If `n` is too large to fit in a [usize].
/// # Example
/// ```
/// use pmath::numth::partition::partition_p;
///
/// // Partitions of 5:
/// // {5}
/// // {4, 1}
/// // {3, 2}
/// // {3, 1, 1}
/// // {2, 2, 1}
/// // {2, 1, 1, 1}
/// // {1, 1, 1, 1, 1}
/// // p(5) = 7
/// assert_eq!(partition_p(5), 7);
/// ```
pub fn partition_p<T>(n: T) -> T
where
    T: PrimInt + ConstZero + ConstOne,
{
    if n < T::ZERO {
        return T::ZERO;
    }
    // since calculating p(n) also requires calculating p of every integer less than n,
    // we can just calculate all values and get the value of p(n) from the vector
    // (last value)
    partition_p_0_to_n(n).pop().unwrap()
}

/// Partition function of integers from `0` to `n`.
/// # Arguments
/// * `n` - The integer up to which to calculate the partition function.
/// # Returns
/// * The partition function of integers from `0` to `n`.
///   Index represents the integer,
///   and the value at that index is the partition function of that integer.
/// # Panics
/// * If `n` cannot be converted to [usize].
/// # Example
/// ```
/// use pmath::numth::partition::partition_p_0_to_n;
///
/// assert_eq!(partition_p_0_to_n(10), vec![1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42]);
/// ```
pub fn partition_p_0_to_n<T>(n: T) -> Vec<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    // get n as usize
    let n = n.to_usize().expect("Cannot convert n to usize.");

    // if n is 0, return 1
    if n == 0 {
        return vec![T::ONE];
    }

    let mut partitions = Vec::with_capacity(n + 1);
    partitions.push(T::ONE); // p(0) = 1
    partitions.push(T::ONE); // p(1) = 1

    while partitions.len() <= n {
        // calculate the next value and add it to vector

        let curr_n = partitions.len();
        let mut next_val = T::ZERO;
        for k in 1..=curr_n {
            let left_value = match curr_n.checked_sub((k * (3 * k - 1)) / 2) {
                Some(ind) => partitions[ind],
                None => break, // the greater of the indices is below zero, so any larger k will only be 0, we can break
            };
            let right_value = match curr_n.checked_sub((k * (3 * k + 1)) / 2) {
                Some(ind) => partitions[ind],
                None => T::ZERO,
            };
            let value = left_value + right_value;

            if k % 2 == 0 {
                next_val = next_val - value;
            } else {
                next_val = next_val + value;
            }
        }

        // push the newly calculated value to the vector
        partitions.push(next_val);
    }

    // return the partitions vector
    partitions
}

#[cfg_attr(doc, katexit::katexit)]
/// Prime partition function.
///
/// Prime partition function is defined as the number of ways an integer
/// can be written as a sum of prime numbers.
/// This function is similar to the partition function,
/// but it only counts partitions that consist of prime numbers.
/// # Arguments
/// * `n` - The integer to find the number of prime partitions of.
/// # Returns
/// * The number of prime partitions of the integer.
/// # Panics
/// * If `n` is too large to fit in a [usize].
/// # Notes
/// * If `n` is negative, the function returns `0` (negative integers cannot be partitioned).
/// * If `n` is `0`, the function returns `1` (the empty partition $\\{\\}$).
/// # Example
/// ```
/// use pmath::numth::partition::partition_prime;
///
/// // Prime partitions of 7: {7}, {5, 2}, {3, 2, 2}
/// assert_eq!(partition_prime(7), 3);
/// ```
pub fn partition_prime<T>(n: T) -> T
where
    T: PrimInt + ConstZero + ConstOne,
{
    if n < T::ZERO {
        return T::ZERO;
    }
    // since calculating p(n) also requires calculating p of every number less than n,
    // we just calculate all values and get the value of p(n) from the vector (last value)
    partition_prime_0_to_n(n).pop().unwrap()
}

/// Prime partition function of integers from `0` to `n`.
/// # Arguments
/// * `n` - The integer up to which to calculate the prime partition function.
/// # Returns
/// * The prime partition function of integers from `0` to `n`.
///   Index represents the integer,
///   and the value at that index is the prime partition function of that integer.
/// # Panics
/// * If `n` cannot be converted to [usize].
/// # Example
/// ```
/// use pmath::numth::partition::partition_prime_0_to_n;
///
/// assert_eq!(partition_prime_0_to_n(10), vec![1, 0, 1, 1, 1, 2, 2, 3, 3, 4, 5]);
/// ```
pub fn partition_prime_0_to_n<T>(n: T) -> Vec<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    let n = n.to_usize().expect("Cannot convert n to usize.");
    let primes = sieve_of_eratosthenes(n);

    let mut dp = vec![T::ZERO; n + 1];

    // 0 can be represented in 1 way = {}
    // (1 can't be represented in the same way, so dp[1] stays 0)
    dp[0] = T::ONE;

    for prime in primes {
        for i in prime..=n {
            dp[i] = dp[i] + dp[i - prime];
        }
    }

    dp
}

#[cfg(test)]
mod tests {
    use super::*;

    const PARTITION_P_TEST_SEQ: [usize; 41] = [
        1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627, 792,
        1002, 1255, 1575, 1958, 2436, 3010, 3718, 4565, 5604, 6842, 8349, 10143, 12310, 14883,
        17977, 21637, 26015, 31185, 37338,
    ];
    const PARTITION_PRIME_TEST_SEQ: [usize; 41] = [
        1, 0, 1, 1, 1, 2, 2, 3, 3, 4, 5, 6, 7, 9, 10, 12, 14, 17, 19, 23, 26, 30, 35, 40, 46, 52,
        60, 67, 77, 87, 98, 111, 124, 140, 157, 175, 197, 219, 244, 272, 302,
    ];

    // partition_p

    #[test]
    fn partition_p_primitive_types() {
        //! Test that [partition_p] works with primitive integer types.

        // unsigned types
        assert_eq!(partition_p(5u8), 7);
        assert_eq!(partition_p(5u16), 7);
        assert_eq!(partition_p(5u32), 7);
        assert_eq!(partition_p(5u64), 7);
        assert_eq!(partition_p(5u128), 7);
        assert_eq!(partition_p(5usize), 7);

        // signed types
        assert_eq!(partition_p(5i8), 7);
        assert_eq!(partition_p(5i16), 7);
        assert_eq!(partition_p(5i32), 7);
        assert_eq!(partition_p(5i64), 7);
        assert_eq!(partition_p(5i128), 7);
        assert_eq!(partition_p(5isize), 7);
    }

    #[test]
    fn partition_p_negative() {
        //! Test that [partition_p] returns correct value 0 for negative inputs.
        assert_eq!(partition_p(-1), 0);
        assert_eq!(partition_p(-10), 0);
    }

    #[test]
    fn partition_p_verify() {
        //! Verify the correctness of [partition_p].
        for (i, val) in PARTITION_P_TEST_SEQ.into_iter().enumerate() {
            assert_eq!(partition_p(i), val);
        }
    }

    // partition_p_0_to_n

    #[test]
    fn partition_p_0_to_n_primitive_types() {
        //! Test that [partition_p_0_to_n] works with primitive integer types.
        // unsigned types
        assert_eq!(partition_p_0_to_n(5u8), vec![1, 1, 2, 3, 5, 7]);
        assert_eq!(partition_p_0_to_n(5u16), vec![1, 1, 2, 3, 5, 7]);
        assert_eq!(partition_p_0_to_n(5u32), vec![1, 1, 2, 3, 5, 7]);
        assert_eq!(partition_p_0_to_n(5u64), vec![1, 1, 2, 3, 5, 7]);
        assert_eq!(partition_p_0_to_n(5u128), vec![1, 1, 2, 3, 5, 7]);
        assert_eq!(partition_p_0_to_n(5usize), vec![1, 1, 2, 3, 5, 7]);

        // signed types
        assert_eq!(partition_p_0_to_n(5i8), vec![1, 1, 2, 3, 5, 7]);
        assert_eq!(partition_p_0_to_n(5i16), vec![1, 1, 2, 3, 5, 7]);
        assert_eq!(partition_p_0_to_n(5i32), vec![1, 1, 2, 3, 5, 7]);
        assert_eq!(partition_p_0_to_n(5i64), vec![1, 1, 2, 3, 5, 7]);
        assert_eq!(partition_p_0_to_n(5i128), vec![1, 1, 2, 3, 5, 7]);
        assert_eq!(partition_p_0_to_n(5isize), vec![1, 1, 2, 3, 5, 7]);
    }

    #[test]
    #[should_panic]
    fn partition_p_0_to_n_negative() {
        //! Test that [partition_p_0_to_n] panics for negative inputs.
        partition_p_0_to_n(-1);
    }

    #[test]
    fn partition_p_0_to_n_verify() {
        //! Verify the correctness of [partition_p_0_to_n].
        for i in 0..PARTITION_P_TEST_SEQ.len() {
            let calculated = partition_p_0_to_n(i);
            let actual = PARTITION_P_TEST_SEQ[0..=i].to_vec();
            assert_eq!(calculated, actual);
        }
    }

    // partition_prime

    #[test]
    fn partition_prime_primitive_types() {
        //! Test that [partition_prime] works with primitive integer types.

        // unsigned types
        assert_eq!(partition_prime(7u8), 3);
        assert_eq!(partition_prime(7u16), 3);
        assert_eq!(partition_prime(7u32), 3);
        assert_eq!(partition_prime(7u64), 3);
        assert_eq!(partition_prime(7u128), 3);
        assert_eq!(partition_prime(7usize), 3);

        // signed types
        assert_eq!(partition_prime(7i8), 3);
        assert_eq!(partition_prime(7i16), 3);
        assert_eq!(partition_prime(7i32), 3);
        assert_eq!(partition_prime(7i64), 3);
        assert_eq!(partition_prime(7i128), 3);
        assert_eq!(partition_prime(7isize), 3);
    }

    #[test]
    fn partition_prime_negative() {
        //! Test that [partition_prime] returns 0 when given negative input.
        assert_eq!(partition_prime(-1), 0);
        assert_eq!(partition_prime(-10), 0);
    }

    #[test]
    fn partition_prime_verify() {
        //! Verify the correctness of [partition_prime].
        for (i, val) in PARTITION_PRIME_TEST_SEQ.into_iter().enumerate() {
            assert_eq!(partition_prime(i), val);
        }
    }

    // partition_prime_0_to_n

    #[test]
    fn partition_prime_0_to_n_primitive_types() {
        //! Test that [partition_prime_0_to_n] works with primitive integer types.

        // unsigned types
        assert_eq!(partition_prime_0_to_n(5u8), vec![1, 0, 1, 1, 1, 2]);
        assert_eq!(partition_prime_0_to_n(5u16), vec![1, 0, 1, 1, 1, 2]);
        assert_eq!(partition_prime_0_to_n(5u32), vec![1, 0, 1, 1, 1, 2]);
        assert_eq!(partition_prime_0_to_n(5u64), vec![1, 0, 1, 1, 1, 2]);
        assert_eq!(partition_prime_0_to_n(5u128), vec![1, 0, 1, 1, 1, 2]);
        assert_eq!(partition_prime_0_to_n(5usize), vec![1, 0, 1, 1, 1, 2]);

        // signed types
        assert_eq!(partition_prime_0_to_n(5i8), vec![1, 0, 1, 1, 1, 2]);
        assert_eq!(partition_prime_0_to_n(5i16), vec![1, 0, 1, 1, 1, 2]);
        assert_eq!(partition_prime_0_to_n(5i32), vec![1, 0, 1, 1, 1, 2]);
        assert_eq!(partition_prime_0_to_n(5i64), vec![1, 0, 1, 1, 1, 2]);
        assert_eq!(partition_prime_0_to_n(5i128), vec![1, 0, 1, 1, 1, 2]);
        assert_eq!(partition_prime_0_to_n(5isize), vec![1, 0, 1, 1, 1, 2]);
    }

    #[test]
    #[should_panic]
    fn partition_prime_0_to_n_negative() {
        //! Test that [partition_prime_0_to_n] panics when given negative input.
        partition_prime_0_to_n(-1);
    }

    #[test]
    fn partition_prime_0_to_n_verify() {
        //! Verify the correctness of [partition_prime_0_to_n].
        for i in 0..PARTITION_PRIME_TEST_SEQ.len() {
            let calculated = partition_prime_0_to_n(i);
            let actual = PARTITION_PRIME_TEST_SEQ[0..=i].to_vec();
            assert_eq!(calculated, actual);
        }
    }
}
