use num_traits::{ConstOne, ConstZero, PrimInt};
use crate::numth::prime::sieve_of_eratosthenes;

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