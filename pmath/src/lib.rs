#![doc = include_str!("../README.md")]

pub mod digits;
pub mod factors;
pub mod geometry;
pub mod linalg;
pub mod primes;
pub mod probability;
pub mod sequences;
pub mod statistics;

use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter;
use std::mem;

use factors::distinct_prime_factors;
use malachite::Integer;
use malachite::base::num::basic::traits::{One, Zero};
use malachite::rational::Rational;
use num_traits::{ConstOne, ConstZero, Euclid, PrimInt, Signed, ToPrimitive};
use primes::sieve_of_eratosthenes;

#[cfg_attr(doc, katexit::katexit)]
/// Simple continued fraction.
///
/// A simple continued fraction is a continued fraction
/// with all numerators equal to `1`.
///
/// If it is finite, it is of the form:
/// $$
///     a\_0 + \\frac{1}{a\_1 + \\frac{1}{a\_2 + \\frac{1}{\\ddots + \\frac{1}{a\_n}}}}
/// $$
/// usually represented by coefficients:
/// $$
///    \\left[ a\_0; a\_1, a\_2, \\ldots, a\_n \\right]
/// $$
/// If it is infinite, it is of the form:
/// $$
///    a\_0 + \\frac{1}{a\_1 + \\frac{1}{a\_2 + \\frac{1}{\\ddots}}}
/// $$
/// usually represented by coefficients:
/// $$
///   \\left[ a\_0; a\_1, a\_2, \\ldots \\right]
/// $$
/// # Example
/// ```
/// use pmath::SimpleContinuedFraction;
///
/// // continued fraction of sqrt(2): [1; 2, 2, 2, ...]
/// //     - coefficient 1 is not repeating
/// //     - coefficient 2 is repeating (forms a periodic part)
/// let cf = SimpleContinuedFraction::from_sqrt(2);
/// assert_eq!(cf.non_periodic(), vec![1].as_slice());
/// assert_eq!(cf.periodic(), Some(vec![2].as_slice()));
///
/// // continued fraction of sqrt(3): [1; 1, 2, 1, 2, ...]
/// //     - coefficient 1 is not repeating
/// //     - coefficients 1 and 2 are repeating (form a periodic part)
/// let cf = SimpleContinuedFraction::from_sqrt(3);
/// assert_eq!(cf.non_periodic(), vec![1].as_slice());
/// assert_eq!(cf.periodic(), Some(vec![1, 2].as_slice()));
///
/// // custom coefficients can also be used:
/// let cf = SimpleContinuedFraction::new(vec![1, 2, 3], Some(vec![4, 5]));
/// assert_eq!(cf.non_periodic(), vec![1, 2, 3].as_slice());
/// assert_eq!(cf.periodic(), Some(vec![4, 5].as_slice()));
///
/// // if there is no periodic part, the continued fraction is finite:
/// let cf = SimpleContinuedFraction::new(vec![1, 2, 3], None);
/// assert_eq!(cf.non_periodic(), vec![1, 2, 3].as_slice());
/// assert_eq!(cf.periodic(), None);
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SimpleContinuedFraction<T> {
    non_periodic: Vec<T>,
    periodic: Option<Vec<T>>,
}
impl<T> SimpleContinuedFraction<T>
where
    T: PrimInt + ConstZero + ConstOne + Into<Integer>,
{
    /// Create a new simple continued fraction.
    /// # Arguments
    /// * `non_periodic` - The non-repeating coefficients of the continued fraction $( a\_0, a\_1, a\_2, \\ldots, a\_k )$.
    /// * `periodic` - The repeating coefficients of the continued fraction $( a\_{k+1}, \\ldots, a\_{k+l} )$, if any.
    /// # Returns
    /// * A new simple continued fraction.
    pub fn new<U, V>(non_periodic: U, periodic: Option<U>) -> Self
    where
        U: IntoIterator<Item = V>,
        V: Borrow<T>,
    {
        let non_periodic = non_periodic.into_iter().map(|x| *x.borrow()).collect();
        let periodic = periodic.map(|p| p.into_iter().map(|x| *x.borrow()).collect());
        Self {
            non_periodic,
            periodic,
        }
    }

    /// Create a new simple continued fraction of the square root of an integer.
    /// # Arguments
    /// * `n` - The integer to create the continued fraction of its square root.
    /// # Returns
    /// * A new simple continued fraction representing the square root of the integer.
    /// # Panics
    /// * If `n` is negative.
    /// * If `n` cannot be converted to [f64].
    pub fn from_sqrt(n: T) -> Self
    where
        T: Hash,
    {
        if n < T::ZERO {
            panic!("Cannot calculate square root of a negative integer.");
        }

        // integer square root of n
        let root = T::from(n.to_f64().expect("Cannot convert n to f64.").sqrt().floor()).unwrap();

        let non_periodic = vec![root];
        let mut periodic = None;

        // if n is not a perfect square, then find the periodic part of the continued fraction
        if root * root != n {
            // vector for storing the periodic part of the continued fraction
            periodic = Some(Vec::new());

            // set for storing the rational part of the numerator and the denominator
            // used for detecting the period
            let mut set = HashSet::new();

            // rational part of the numerator, it is a negative number such that -root < num < 0
            // here it is stored as positive because calculations take into account the negative sign
            let mut num = root;
            // denominator, starts with 1
            let mut denom = T::ONE;

            // calculate the next iteration of the continued fraction
            // until the set contains the numerator and the denominator
            // which means the period is found
            while !set.contains(&(num, denom)) {
                set.insert((num, denom));

                denom = (n - num * num) / denom;
                let expanded_val = (num + root) / denom;

                // push the expanded value to the periodic part of the continued fraction
                periodic.as_mut().unwrap().push(expanded_val);

                num = denom * expanded_val - num; // -(num - denom * expanded_val)
            }
        }

        Self {
            non_periodic,
            periodic,
        }
    }

    /// The non-repeating coefficients of the continued fraction.
    /// # Returns
    /// * A slice of the non-repeating coefficients.
    pub fn non_periodic(&self) -> &[T] {
        &self.non_periodic
    }

    /// The repeating coefficients of the continued fraction, if any.
    /// # Returns
    /// * An [Option] containing a slice of the repeating coefficients.
    pub fn periodic(&self) -> Option<&[T]> {
        self.periodic.as_deref()
    }

    /// The convergents of the continued fraction.
    ///
    /// These are the fractions that approximate the value of the continued fraction,
    /// and are generated by taking the coefficients of the continued fraction:
    /// $$
    ///     \\begin{align*}
    ///         &a\_0 \\\\
    ///         &a\_0 + \\frac{1}{a\_1} \\\\
    ///         &a\_0 + \\frac{1}{a\_1 + \\frac{1}{a\_2}} \\\\
    ///         &a\_0 + \\frac{1}{a\_1 + \\frac{1}{a\_2 + \\frac{1}{a\_3}}} \\\\
    ///         &\\vdots
    ///     \\end{align*}
    /// $$
    /// Each subsequent convergent uses one more coefficient than the previous one
    /// therefore better approximating the value of the continued fraction.
    /// # Returns
    /// * An iterator over the convergents of the continued fraction.
    ///   If the continued fraction is finite, the iterator ends with
    ///   the exact value of the continued fraction, and
    ///   if it is infinite, the iterator continues indefinitely,
    ///   producing fractions that better and better approximate the value of the continued fraction.
    /// # Example
    /// ```
    /// use pmath::SimpleContinuedFraction;
    /// use malachite::rational::Rational;
    ///
    /// let cf = SimpleContinuedFraction::new(vec![1, 2], Some(vec![3, 4]));
    /// let mut convergents = cf.convergents();
    /// assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(1, 1));
    /// assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(3, 2));
    /// assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(10, 7));
    /// assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(43, 30));
    /// assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(139, 97));
    /// assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(599, 418));
    /// // ... and so on (infinitely)
    /// ```
    pub fn convergents(&self) -> impl Iterator<Item = Rational> {
        let mut prev_num = Integer::ZERO;
        let mut prev_den = Integer::ONE;
        let mut num = Integer::ONE;
        let mut den = Integer::ZERO;
        let mut values = self
            .non_periodic
            .iter()
            .chain(self.periodic.iter().flat_map(|v| v.iter().cycle()));

        iter::from_fn(move || {
            let next_value = values.next()?;
            let next_num = (*next_value).into() * &num + &prev_num;
            let next_den = (*next_value).into() * &den + &prev_den;
            prev_num = mem::replace(&mut num, next_num);
            prev_den = mem::replace(&mut den, next_den);
            Some(Rational::from_integers_ref(&num, &den))
        })
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// Multiplicative order.
///
/// Multiplicative order of an integer `a` modulo `n`, where `a` and `n` are coprime,
/// is the smallest positive integer `k` such that $ a^k \equiv 1 \pmod n $.
/// # Arguments
/// * `a` - The base.
/// * `n` - The modulus.
/// # Returns
/// * The multiplicative order.
/// # Panics
/// * If `a` or `n` is less than `2`.
/// * If `a` and `n` are not coprime.
/// # Example
/// ```
/// use pmath::ord;
///
/// // ord(3, 7) = 6
/// assert_eq!(ord(3, 7), 6);
/// ```
pub fn ord<T>(a: T, n: T) -> T
where
    T: PrimInt + ConstOne,
{
    let t2 = T::from(2).unwrap();
    if n < t2 || a < t2 {
        panic!("a and n must be greater than or equal to 2.");
    }
    // we want the smallest k so that a^k ≡ 1 (mod n)
    // a^k (mod n) = ((a^(k-1) (mod n)) * a) (mod n)
    // example: 8^2 mod 7 = ((8 mod 7) * 8) mod 7
    // k <= n - 1 (Fermat's little theorem)

    let mut result = T::ONE;
    let mut k = T::ONE;
    while k < n {
        result = (result * a) % n;
        if result == T::ONE {
            return k;
        }
        k = k + T::ONE;
    }

    // because of fermat's little theorem, if a and n are coprime,
    // the multiplicative order must exist because a^(n-1) ≡ 1 (mod n)
    // if we reach this point, it means we didn't find the order,
    // so a and n are not coprime
    panic!("a and n are not coprime.");
}

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
/// use pmath::partition_p;
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
/// use pmath::partition_p_0_to_n;
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
/// use pmath::partition_prime;
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
/// use pmath::partition_prime_0_to_n;
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

#[cfg_attr(doc, katexit::katexit)]
/// Factorial of an integer.
///
/// Factorial function is defined for non-negative integers as:
/// $$
///     n! =
///     \\begin{cases}
///         1 & \\text{if}\\quad n = 0 \\\\
///         \\prod\_{i=1}\^{n} i & \\text{if}\\quad n > 0
///     \\end{cases}
/// $$
/// # Arguments
/// * `n` - The integer to find the factorial of.
/// # Returns
/// * The factorial.
/// # Panics
/// * If `n` is negative.
/// # Example
/// ```
/// use pmath::factorial;
///
/// // 5! = 120
/// assert_eq!(factorial(5), 120);
/// // 0! = 1
/// assert_eq!(factorial(0), 1);
/// ```
pub fn factorial<T>(n: T) -> T
where
    T: PrimInt + ConstOne,
{
    let mut fact = T::ONE;
    let mut i = T::ONE;
    while i < n {
        i = i + T::ONE;
        fact = fact * i;
    }
    fact
}

/// Factorials of integers from `0` to `n`.
/// # Arguments
/// * `n` - The integer up to which to calculate the factorials.
/// # Returns
/// * The factorials of integers from `0` to `n`.
///   Index represents the integer, and the value at that index is the factorial of that integer.
/// # Panics
/// * If `n` cannot be converted to [usize].
/// # Example
/// ```
/// use pmath::factorial_0_to_n;
///
/// assert_eq!(factorial_0_to_n(5), vec![1, 1, 2, 6, 24, 120]);
/// ```
pub fn factorial_0_to_n<T>(n: T) -> Vec<T>
where
    T: PrimInt + ConstOne,
{
    let n = n.to_usize().expect("Cannot convert n to usize.");
    let mut factorials = vec![T::ONE; n + 1];
    for i in 2..=n {
        factorials[i] = factorials[i - 1] * T::from(i).unwrap();
    }
    factorials
}

/// Integer square root.
///
/// Square root of an integer rounded down to the nearest integer.
/// Slower than casting to [f64] and using `.sqrt().floor()`.
/// To be used with big integers which would lose precision if cast to [f64].
/// # Arguments
/// * `n` - The integer to find the integer square root of.
/// # Returns
/// * The integer square root.
/// # Panics
/// * If `n` is negative.
/// # Example
/// ```
/// use pmath::isqrt;
///
/// // isqrt of 12 is 3
/// assert_eq!(isqrt(12), 3);
/// ```
pub fn isqrt<T>(n: T) -> T
where
    T: PrimInt + ConstZero + ConstOne,
{
    if n < T::ZERO {
        panic!("Cannot calculate square root of a negative integer.");
    } else if n <= T::ONE {
        n
    } else {
        let t2 = T::from(2).unwrap();
        let mut x0 = t2.pow((n.to_u128().unwrap().ilog2() / 2) + 1);
        let mut x1 = (x0 + n / x0) / t2;
        while x1 < x0 {
            x0 = x1;
            x1 = (x0 + n / x0) / t2;
        }
        x0
    }
}

/// The greatest common divisor of two integers.
///
/// Calculated using the Euclidean algorithm.
/// If both integers are `0`, the result is `0`.
/// # Arguments
/// * `num1` - The first integer.
/// * `num2` - The second integer.
/// # Returns
/// * The greatest common divisor.
/// # Panics
/// * If either of the integers is negative.
/// # Example
/// ```
/// use pmath::gcd;
///
/// // gcd of 12 and 18 is 6
/// assert_eq!(gcd(12, 18), 6);
/// // gcd of 0 and 0 is 0
/// assert_eq!(gcd(0, 0), 0);
/// // gcd of 0 and 5 is 5
/// assert_eq!(gcd(0, 5), 5);
/// ```
pub fn gcd<T>(mut num1: T, mut num2: T) -> T
where
    T: PrimInt + ConstZero,
{
    if num1 < T::ZERO || num2 < T::ZERO {
        panic!("Cannot calculate GCD of negative numbers.");
    }
    if num1 < num2 {
        (num1, num2) = (num2, num1);
    }
    while num2 > T::ZERO {
        (num1, num2) = (num2, num1 % num2);
    }
    num1
}

/// The greatest common divisor of multiple integers.
/// # Arguments
/// * `nums` - The integers to calculate the GCD of.
/// # Returns
/// * The greatest common divisor.
/// # Panics
/// * If any of the integers are negative.
/// # Example
/// ```
/// use pmath::gcd_multiple;
///
/// // gcd of 12, 18 and 24 is 6
/// assert_eq!(gcd_multiple([12, 18, 24]), 6);
/// ```
pub fn gcd_multiple<T, U, I>(nums: I) -> T
where
    T: PrimInt + ConstZero,
    U: Borrow<T>,
    I: IntoIterator<Item = U>,
{
    let mut nums = nums.into_iter();
    let n1 = match nums.next() {
        Some(x) => *x.borrow(),
        None => T::ZERO,
    };
    let n2 = match nums.next() {
        Some(x) => *x.borrow(),
        None => T::ZERO,
    };
    let mut result = gcd(n1, n2);
    for n in nums {
        result = gcd(result, *n.borrow());
    }
    result
}

#[cfg_attr(doc, katexit::katexit)]
/// The greatest common divisor and coefficients of Bézout's identity of two integers.
///
/// Bézout's identity states that for any non-negative integers $a$ and $b$,
/// there exist integers $x$ and $y$ such that:
/// $$
///     ax + by = \\text{gcd}(a, b)
/// $$
///
/// Calculated using the extended Euclidean algorithm.
/// # Arguments
/// * $a$ - The first integer.
/// * $b$ - The second integer.
/// # Returns
/// * $\\text{gcd}(a, b)$ - The greatest common divisor.
/// * $x$ - The coefficient of $a$ in Bézout's identity.
/// * $y$ - The coefficient of $b$ in Bézout's identity.
/// # Panics
/// * If either of the integers is negative.
/// # Example
/// ```
/// use pmath::gcd_extended;
///
/// assert_eq!(gcd_extended(12, 18), (6, -1, 1));  // 12 * -1 + 18 * 1 = 6
/// assert_eq!(gcd_extended(0, 0), (0, 1, 0));     // 0 * 1 + 0 * 0 = 0
/// assert_eq!(gcd_extended(0, 5), (5, 0, 1));     // 0 * 0 + 5 * 1 = 5
/// ```
pub fn gcd_extended<T>(a: T, b: T) -> (T, T, T)
where
    T: PrimInt + ConstZero + ConstOne + Signed,
{
    if a < T::ZERO || b < T::ZERO {
        panic!("Cannot calculate GCD of negative numbers.");
    }

    let mut r0 = a;
    let mut r1 = b;
    let mut switch = false;
    if r0 < r1 {
        (r0, r1) = (r1, r0);
        switch = true;
    }
    let mut s0 = T::ONE;
    let mut s1 = T::ZERO;
    let mut t0 = T::ZERO;
    let mut t1 = T::ONE;

    while r1 > T::ZERO {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (s0, s1) = (s1, s0 - q * s1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    if switch {
        (s0, t0) = (t0, s0);
    }
    (r0, s0, t0)
}

/// The least common multiple of two integers.
///
/// If either of the integers is `0`, the result is `0`.
/// # Arguments
/// * `n1` - The first integer.
/// * `n2` - The second integer.
/// # Returns
/// * The least common multiple.
/// # Panics
/// * If either of the integers is negative.
/// # Example
/// ```
/// use pmath::lcm;
///
/// // lcm of 12 and 18 is 36
/// assert_eq!(lcm(12, 18), 36);
/// // lcm of 0 and 5 is 0
/// assert_eq!(lcm(0, 5), 0);
/// // lcm of 0 and 0 is 0
/// assert_eq!(lcm(0, 0), 0);
/// ```
pub fn lcm<T>(n1: T, n2: T) -> T
where
    T: PrimInt + ConstZero,
{
    let gcd = gcd(n1, n2);
    if gcd == T::ZERO {
        T::ZERO
    } else {
        (n1 / gcd) * n2
    }
}

/// The least common multiple of multiple integers.
/// # Arguments
/// * `nums` - The integers to calculate the LCM of.
/// # Returns
/// * The least common multiple.
/// # Panics
/// * If any of the integers are negative.
/// # Example
/// ```
/// use pmath::lcm_multiple;
///
/// // lcm of 12, 18 and 24 is 72
/// assert_eq!(lcm_multiple([12, 18, 24]), 72);
/// ```
pub fn lcm_multiple<T, U, I>(nums: I) -> T
where
    T: PrimInt + ConstZero,
    U: Borrow<T>,
    I: IntoIterator<Item = U>,
{
    let mut nums = nums.into_iter();
    let n1 = match nums.next() {
        Some(x) => *x.borrow(),
        None => T::ZERO,
    };
    let n2 = match nums.next() {
        Some(x) => *x.borrow(),
        None => T::ZERO,
    };
    let mut result = lcm(n1, n2);
    for n in nums {
        result = lcm(result, *n.borrow());
    }
    result
}

/// Newton's method for finding the zero of a function.
///
/// If the function does not converge to a zero, this might run indefinitely.
/// It is recommended to use this method only for functions that are known to converge.
/// # Arguments
/// * `x0` - The initial guess.
/// * `precision` - The precision of the answer (the error will be less than this).
/// * `function` - The function to find the zero of.
/// * `derivative` - The derivative of the function.
/// # Returns
/// * [Some] with the zero of the function if it converges to a zero within the given precision,
///   or [None] if the value of the derivative is `0` at some of the evaluated points.
/// # Panics
/// * If `x0` cannot be converted to [f64].
/// # Example
/// ```
/// use pmath::newtons_method;
///
/// // f(x) = x^2 - 2
/// // The zero of f(x) is the square root of 2.
/// let x0 = 1.0;
/// let precision = 1e-10;
/// let function = |x| x * x - 2.0;
/// let derivative = |x| 2.0 * x;
/// let calculated_zero = newtons_method(x0, precision, function, derivative).unwrap();
/// assert!((calculated_zero - 2.0_f64.sqrt()).abs() < precision);
/// ```
pub fn newtons_method<T, F, D>(x0: T, precision: f64, function: F, derivative: D) -> Option<f64>
where
    T: ToPrimitive,
    F: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    let mut x = x0.to_f64().expect("Cannot convert x0 to f64.");
    let mut prev_x = f64::NEG_INFINITY;

    while (x - prev_x).abs() > precision {
        prev_x = x;
        let derivative_value = derivative(prev_x);
        if derivative_value == 0.0 {
            return None; // derivative is zero, cannot proceed
        }
        x = prev_x - function(prev_x) / derivative_value;
    }

    Some(x)
}

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

#[cfg_attr(doc, katexit::katexit)]
/// Congruence relation.
///
/// A congruence relation is an equation that states that two integers
/// are congruent modulo a third integer.
/// Two integers $x$ and $y$ are said to be congruent modulo $n$
/// if they have the same remainder $a$ when divided by $n$.
/// This is written as:
/// $$ x \equiv a \pmod n \\\\
///    y \equiv a \pmod n
/// $$
/// # Example
/// ```
/// use pmath::CongruenceRelation;
///
/// let c1 = CongruenceRelation::new(3, 5); // 3 === 3 (mod 5)
/// let c2 = CongruenceRelation::new(8, 5); // 8 === 3 (mod 5)
/// assert_eq!(*c1.a(), 3);
/// assert_eq!(*c1.n(), 5);
/// assert_eq!(*c2.a(), 3);
/// assert_eq!(*c2.n(), 5);
/// assert_eq!(c1, c2);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CongruenceRelation<T> {
    a: T,
    n: T,
}
impl<T> CongruenceRelation<T>
where
    T: PartialOrd + Euclid + ConstZero,
{
    /// Create a new [CongruenceRelation].
    /// # Arguments
    /// * `a` - The remainder.
    /// * `n` - The modulus (must be positive).
    /// # Panics
    /// * If `n` is not positive.
    pub fn new(a: T, n: T) -> Self {
        if n <= T::ZERO {
            panic!("Modulus must be positive.");
        }
        Self { a: a.rem_euclid(&n), n }
    }

    /// Get the remainder of the congruence relation.
    ///
    /// This might not be the same as the $a$ provided to [CongruenceRelation::new]
    /// as it is reduced modulo $n$.
    /// # Returns
    /// * The remainder.
    pub fn a(&self) -> &T {
        &self.a
    }

    /// Get the modulus of the congruence relation.
    /// # Returns
    /// * The modulus.
    pub fn n(&self) -> &T {
        &self.n
    }
}

/// Solve a system of linear congruences.
///
/// Uses the Chinese remainder theorem.
/// If a solution exists, it is unique modulo the least common multiple of the moduli.
///
/// The moduli do not need to be coprime, but if they are not, a solution might not exist.
/// # Arguments
/// * `congruences` - An iterable of [CongruenceRelation]s representing
///   the system of linear congruences.
/// # Returns
/// * [Some] with the solution if it exists,
///   or [None] if no solution exists or no congruences were provided.
/// # Panics
/// * If any of the moduli are negative.
/// # Example
/// ```
/// use pmath::{system_of_linear_congruences, CongruenceRelation};
///
/// let congruences = [
///    CongruenceRelation::new(9, 10),
///    CongruenceRelation::new(5, 6),
/// ];
/// assert_eq!(system_of_linear_congruences(congruences), Some(29));
///
/// let congruences = [
///    CongruenceRelation::new(7i64, 19),
///    CongruenceRelation::new(6, 17),
///    CongruenceRelation::new(11, 13),
///    CongruenceRelation::new(2, 7),
///    CongruenceRelation::new(2, 5),
///    CongruenceRelation::new(1, 3),
///    CongruenceRelation::new(4, 11),
/// ];
/// assert_eq!(system_of_linear_congruences(congruences), Some(3_903_937));
/// ```
pub fn system_of_linear_congruences<T, U, V>(congruences: U) -> Option<T>
where
    U: IntoIterator<Item = V>,
    V: Borrow<CongruenceRelation<T>>,
    T: Copy + PrimInt + ConstOne + ConstZero + Euclid + PartialOrd + Signed
{
    let mut congruences = congruences
        .into_iter()
        .map(|val| *val.borrow());

    let mut a;
    let mut n;
    match congruences.next() {
        Some(congruence) => {
            a = *congruence.a();
            n = *congruence.n();
        },
        None => return None, // no congruences provided
    }

    for congruence in congruences {
        let mut a1 = a;
        let a2 = *congruence.a();
        let mut n1 = n;
        let n2 = *congruence.n();
        let gcd = gcd(n1, n2);

        // a1 % gcd != a2 % gcd
        // this would imply that the solution x has to have 2 different remainders
        // when divided by gcd, so no solution exists
        if (a2 - a1).rem_euclid(&gcd) != T::ZERO {
            return None;
        }

        // reduce the problem
        n1 = n1 / gcd;
        a1 = a1.rem_euclid(&n1);

        let (_, m1, m2) = gcd_extended(n1, n2);

        let new_n = n1 * n2;
        a = (a1 * n2 * m2 + a2 * n1 * m1).rem_euclid(&new_n);
        n = new_n;
    }

    Some(a)
}
