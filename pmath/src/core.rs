//! Core utilities

use std::borrow::Borrow;
use num_traits::{ConstOne, ConstZero, PrimInt, Signed};

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
    T: PrimInt + ConstOne + ConstZero,
{
    if n < T::ZERO {
        panic!("negative factorial");
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    // factorial

    #[test]
    #[should_panic]
    fn factorial_negative() {
        factorial(-1);
    }

    #[test]
    fn factorial_primitive_types() {
        // unsigned types
        assert_eq!(factorial(5u8), 120);
        assert_eq!(factorial(5u16), 120);
        assert_eq!(factorial(5u32), 120);
        assert_eq!(factorial(5u64), 120);
        assert_eq!(factorial(5u128), 120);
        assert_eq!(factorial(5usize), 120);

        // signed types
        assert_eq!(factorial(5i8), 120);
        assert_eq!(factorial(5i16), 120);
        assert_eq!(factorial(5i32), 120);
        assert_eq!(factorial(5i64), 120);
        assert_eq!(factorial(5i128), 120);
        assert_eq!(factorial(5isize), 120);
    }

    #[test]
    fn factorial_verify() {
        let mut f = 1;
        assert_eq!(factorial(0), f);
        for i in 1..=10 {
            f *= i;
            assert_eq!(factorial(i), f);
        }
    }

    // factorial 0 to n

    #[test]
    #[should_panic]
    fn factorial_0_to_n_negative() {
        factorial_0_to_n(-1);
    }

    #[test]
    fn factorial_0_to_n_primitive_types() {
        // unsigned types
        assert_eq!(factorial_0_to_n(5u8)[5], 120);
        assert_eq!(factorial_0_to_n(5u16)[5], 120);
        assert_eq!(factorial_0_to_n(5u32)[5], 120);
        assert_eq!(factorial_0_to_n(5u64)[5], 120);
        assert_eq!(factorial_0_to_n(5u128)[5], 120);
        assert_eq!(factorial_0_to_n(5usize)[5], 120);

        // signed types
        assert_eq!(factorial_0_to_n(5i8)[5], 120);
        assert_eq!(factorial_0_to_n(5i16)[5], 120);
        assert_eq!(factorial_0_to_n(5i32)[5], 120);
        assert_eq!(factorial_0_to_n(5i64)[5], 120);
        assert_eq!(factorial_0_to_n(5i128)[5], 120);
        assert_eq!(factorial_0_to_n(5isize)[5], 120);
    }

    #[test]
    fn factorial_0_to_n_verify() {
        assert_eq!(factorial_0_to_n(0), vec![1]);
        assert_eq!(factorial_0_to_n(5), vec![1, 1, 2, 6, 24, 120]);
    }

    // isqrt

    #[test]
    fn isqrt_verify() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(1), 1);
        assert_eq!(isqrt(2), 1);
        assert_eq!(isqrt(3), 1);
        assert_eq!(isqrt(4), 2);
        assert_eq!(isqrt(5), 2);
        assert_eq!(isqrt(6), 2);
        assert_eq!(isqrt(7), 2);
        assert_eq!(isqrt(8), 2);
        assert_eq!(isqrt(9), 3);
        assert_eq!(isqrt(10), 3);
        assert_eq!(isqrt(11), 3);
        assert_eq!(isqrt(12), 3);
        assert_eq!(isqrt(13), 3);
        assert_eq!(isqrt(14), 3);
        assert_eq!(isqrt(15), 3);
        assert_eq!(isqrt(16), 4);
        assert_eq!(isqrt(17), 4);
        assert_eq!(isqrt(18), 4);
        assert_eq!(isqrt(19), 4);
        assert_eq!(isqrt(20), 4);
        assert_eq!(isqrt(21), 4);
        assert_eq!(isqrt(22), 4);
        assert_eq!(isqrt(23), 4);
        assert_eq!(isqrt(24), 4);
        assert_eq!(isqrt(25), 5);
    }

    #[test]
    #[should_panic]
    fn isqrt_negative() {
        isqrt(-1);
    }

    #[test]
    fn isqrt_primitive_types() {
        // unsigned types
        assert_eq!(isqrt(16u8), 4);
        assert_eq!(isqrt(16u16), 4);
        assert_eq!(isqrt(16u32), 4);
        assert_eq!(isqrt(16u64), 4);
        assert_eq!(isqrt(16u128), 4);
        assert_eq!(isqrt(16usize), 4);

        // signed types
        assert_eq!(isqrt(16i8), 4);
        assert_eq!(isqrt(16i16), 4);
        assert_eq!(isqrt(16i32), 4);
        assert_eq!(isqrt(16i64), 4);
        assert_eq!(isqrt(16i128), 4);
        assert_eq!(isqrt(16isize), 4);
    }

    // gcd

    #[test]
    #[should_panic]
    fn gcd_negative() {
        gcd(-1, 5);
    }

    #[test]
    fn gcd_zeros() {
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn gcd_zero() {
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(10, 0), 10);
    }

    #[test]
    fn gcd_primitive_types() {
        // unsigned types
        assert_eq!(gcd(12u8, 18u8), 6);
        assert_eq!(gcd(12u16, 18u16), 6);
        assert_eq!(gcd(12u32, 18u32), 6);
        assert_eq!(gcd(12u64, 18u64), 6);
        assert_eq!(gcd(12u128, 18u128), 6);
        assert_eq!(gcd(12usize, 18usize), 6);

        // signed types
        assert_eq!(gcd(12i8, 18i8), 6);
        assert_eq!(gcd(12i16, 18i16), 6);
        assert_eq!(gcd(12i32, 18i32), 6);
        assert_eq!(gcd(12i64, 18i64), 6);
        assert_eq!(gcd(12i128, 18i128), 6);
        assert_eq!(gcd(12isize, 18isize), 6);
    }

    #[test]
    fn gcd_verify() {
        assert_eq!(gcd(25, 5), 5);
        assert_eq!(gcd(250, 5), 5);
        assert_eq!(gcd(12, 52), 4);
        assert_eq!(gcd(52, 12), 4);
        assert_eq!(gcd(342, 456), 114);
    }

    // lcm

    #[test]
    #[should_panic]
    fn lcm_negative() {
        lcm(-1, 5);
    }

    #[test]
    fn lcm_zero() {
        assert_eq!(lcm(0, 10), 0);
        assert_eq!(lcm(10, 0), 0);
        assert_eq!(lcm(0, 0), 0);
    }

    #[test]
    fn lcm_primitive_types() {
        // unsigned types
        assert_eq!(lcm(12u8, 18u8), 36);
        assert_eq!(lcm(12u16, 18u16), 36);
        assert_eq!(lcm(12u32, 18u32), 36);
        assert_eq!(lcm(12u64, 18u64), 36);
        assert_eq!(lcm(12u128, 18u128), 36);
        assert_eq!(lcm(12usize, 18usize), 36);

        // signed types
        assert_eq!(lcm(12i8, 18i8), 36);
        assert_eq!(lcm(12i16, 18i16), 36);
        assert_eq!(lcm(12i32, 18i32), 36);
        assert_eq!(lcm(12i64, 18i64), 36);
        assert_eq!(lcm(12i128, 18i128), 36);
        assert_eq!(lcm(12isize, 18isize), 36);
    }

    #[test]
    fn lcm_verify() {
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(18, 12), 36);
        assert_eq!(lcm(12, 52), 156);
        assert_eq!(lcm(52, 12), 156);
        assert_eq!(lcm(342, 456), 1368);
        assert_eq!(lcm(456, 342), 1368);
        assert_eq!(lcm(1, 66), 66);
        assert_eq!(lcm(66, 1), 66);
    }
}
