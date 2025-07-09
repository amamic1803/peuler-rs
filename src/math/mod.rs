//! Mathematical functions.

pub mod linalg;
pub mod prime;
pub mod sequence;

use std::borrow::Borrow;
use std::collections::HashSet;
use std::iter;
use std::mem;

use prime::{distinct_prime_factors, sieve_of_eratosthenes};

use itertools::Itertools;
use malachite::Integer;
use malachite::base::num::basic::traits::{One, Zero};
use malachite::rational::Rational;
use num_traits::{ConstOne, ConstZero, NumCast, PrimInt, ToPrimitive, Unsigned};

/// Represents a continued fraction.
/// # Example
/// ```
/// use peuler::math::ContinuedFraction;
/// // Continued fraction of sqrt(2): [1; 2, 2, 2, ...]
/// let cf = ContinuedFraction::from_sqrt(2);
/// assert_eq!(cf.non_periodic(), vec![1i64].as_slice());
/// assert_eq!(cf.periodic(), Some(vec![2i64].as_slice()));
/// ```
pub struct ContinuedFraction {
    non_periodic: Vec<i64>,
    periodic: Option<Vec<i64>>,
}
impl ContinuedFraction {
    /// Creates a new continued fraction.
    pub fn new(non_periodic: Vec<i64>, periodic: Option<Vec<i64>>) -> Self {
        Self {
            non_periodic,
            periodic,
        }
    }

    /// Creates the continued fraction by taking the square root of a number.
    pub fn from_sqrt(n: i64) -> Self {
        assert!(n >= 0, "Number must be non-negative.");

        // integer square root of n
        let root = (n as f64).sqrt().floor() as i64;

        let non_periodic = vec![root];
        let mut periodic = None;

        // if n is not a perfect square, then find the periodic part of the continued fraction
        if root * root != n {
            // vector for storing the periodic part of the continued fraction
            periodic = Some(Vec::new());

            // set for storing the rational part of the numerator and the denominator
            // used for detecting the period
            let mut set = HashSet::new();

            // rational part of the numerator, it is negative number such that -root < num < 0
            // here it is stored as positive because calculations take into account the negative sign
            let mut num = root;
            // denominator, starts with 1
            let mut denom = 1;

            // calculate next iteration of the continued fraction
            // until the set contains the numerator and the denominator
            // which means the period is found
            while !set.contains(&(num, denom)) {
                set.insert((num, denom));

                denom = (n - num * num) / denom;
                let expanded_val = (num + root) / denom;

                // push the expanded value to the periodic part of the continued fraction
                periodic.as_mut().unwrap().push(expanded_val);

                num = -(num - denom * expanded_val);
            }
        }

        Self {
            non_periodic,
            periodic,
        }
    }

    /// Returns the reference to the non-periodic part of the continued fraction.
    pub fn non_periodic(&self) -> &[i64] {
        &self.non_periodic
    }

    /// Returns the reference to the periodic part of the continued fraction.
    pub fn periodic(&self) -> Option<&[i64]> {
        self.periodic.as_deref()
    }

    /// Iterator over the convergents of the continued fraction.
    /// If the continued fraction is finite, then the iterator is also finite.
    /// If the continued fraction is infinite, then the iterator is infinite.
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
            let next_num = Integer::const_from_signed(*next_value) * &num + &prev_num;
            let next_den = Integer::const_from_signed(*next_value) * &den + &prev_den;
            prev_num = mem::replace(&mut num, next_num);
            prev_den = mem::replace(&mut den, next_den);
            Some(Rational::from_integers_ref(&num, &den))
        })
    }

    /// Returns the convergent at index n.
    pub fn convergent_n(&self, n: usize) -> Option<Rational> {
        self.convergents().nth(n)
    }
}

/// Returns the iterator over the digits of a number.
/// The iterator iterates from the most significant digit to the least significant digit,
/// but can be reversed easily with `.rev()`.
/// # Arguments
/// * `n` - The number to get the digits of.
/// * `radix` - The radix of the number.
/// # Returns
/// * The iterator over the digits of the number.
/// # Example
/// ```
/// use peuler::math::digits;
///
/// assert_eq!(digits(123, 10).collect::<Vec<u8>>(), vec![1, 2, 3]);
/// assert_eq!(digits(123, 10).rev().collect::<Vec<u8>>(), vec![3, 2, 1]);
/// assert_eq!(digits(0, 10).len(), 0);
/// assert_eq!(digits(123, 10).rev().len(), 3);
/// ```
pub fn digits(n: u64, radix: u8) -> impl DoubleEndedIterator<Item = u8> + ExactSizeIterator {
    struct DigitsIter {
        num: u64,
        radix: u64,
        front_weight: u64,
        length: usize,
    }
    impl DigitsIter {
        fn new(num: u64, radix: u8) -> Self {
            let radix = radix as u64;
            let length;
            let front_weight;
            if num == 0 {
                length = 0;
                front_weight = 0;
            } else {
                length = num.ilog(radix) + 1;
                front_weight = radix.pow(length - 1);
            }
            Self {
                num,
                radix,
                front_weight,
                length: length as usize,
            }
        }
    }
    impl Iterator for DigitsIter {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            if self.num == 0 && self.front_weight == 0 {
                None
            } else {
                let next_digit = self.num / self.front_weight;
                self.num %= self.front_weight;
                self.front_weight /= self.radix;
                self.length -= 1;
                Some(next_digit as Self::Item)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.length, Some(self.length))
        }
    }
    impl DoubleEndedIterator for DigitsIter {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.num == 0 {
                None
            } else {
                let next_digit = self.num % self.radix;
                self.num /= self.radix;
                self.front_weight /= self.radix;
                self.length -= 1;
                Some(next_digit as Self::Item)
            }
        }
    }
    impl ExactSizeIterator for DigitsIter {}

    DigitsIter::new(n, radix)
}

/// Creates an integer from digits.
/// Digits can be any type that implements [IntoIterator].
/// # Arguments
/// * `digits` - The type that implements [IntoIterator] and contains digits.
/// * `radix` - The radix of the number.
/// # Returns
/// * `u64` - The integer.
/// # Example
/// ```
/// use peuler::math::digits_to_int;
/// // 123 -> 123
/// assert_eq!(digits_to_int([1u8, 2u8, 3u8], 10), 123);
/// ```
pub fn digits_to_int<T, U>(digits: T, radix: u8) -> u64
where
    T: IntoIterator<Item = U>,
    U: Borrow<u8>,
{
    let mut result = 0;
    let radix = radix as u64;
    for digit in digits {
        result = result * radix + *digit.borrow() as u64;
    }
    result
}

/// Calculates the factorial of a number.
/// # Arguments
/// * `n` - The number to find the factorial of.
/// # Returns
/// * The factorial of the number.
/// # Example
/// ```
/// use peuler::math::factorial;
/// // 5! = 120
/// assert_eq!(factorial(5u8), 120);
/// ```
pub fn factorial<T>(n: T) -> T
where
    T: PrimInt + Unsigned + ConstOne,
{
    let mut fact = T::ONE;
    let mut i = T::ONE;
    while i < n {
        i = i + T::ONE;
        fact = fact * i;
    }
    fact
}

/// Calculates the factorials of numbers from 1 to n.
/// # Arguments
/// * `n` - The number to find the factorials of.
/// # Returns
/// * `Vec<u64>` - The factorials of numbers from 0 to n. Index represents the number.
/// # Example
/// ```
/// use peuler::math::factorial_1_to_n;
/// assert_eq!(factorial_1_to_n(5u8), vec![1, 1, 2, 6, 24, 120]);
/// ```
pub fn factorial_1_to_n<T>(n: T) -> Vec<u64>
where
    T: PrimInt + Unsigned + ConstOne,
{
    let n = n.to_usize().expect("Number too large.");
    let mut factorials = vec![1; n + 1];
    for i in 2..=n {
        factorials[i] = factorials[i - 1] * (i as u64);
    }
    factorials
}

/// Finds the greatest common divisor of two numbers.
/// Uses the Euclidean algorithm.
/// # Arguments
/// * `num1` - The first number.
/// * `num2` - The second number.
/// # Returns
/// * The greatest common divisor.
/// # Example
/// ```
/// use peuler::math::gcd;
/// // gcd of 12 and 18 is 6
/// assert_eq!(gcd(12u8, 18u8), 6);
/// // gcd of 0 and 0 is 0
/// assert_eq!(gcd(0u8, 0u8), 0);
/// // gcd of 0 and 5 is 5
/// assert_eq!(gcd(0u8, 5u8), 5);
/// ```
pub fn gcd<T>(mut num1: T, mut num2: T) -> T
where
    T: PrimInt + Unsigned + ConstZero,
{
    if num1 < num2 {
        (num1, num2) = (num2, num1);
    }
    while num2 != T::ZERO {
        (num1, num2) = (num2, num1 % num2);
    }
    num1
}

/// Finds the greatest common divisor of multiple numbers.
/// # Arguments
/// * `nums` - The numbers.
/// # Returns
/// * The greatest common divisor.
/// # Panics
/// If there are less than 2 numbers.
/// # Example
/// ```
/// use peuler::math::gcd_multiple;
/// // gcd of 12, 18 and 24 is 6
/// assert_eq!(gcd_multiple([12u8, 18u8, 24u8]), 6);
/// ```
pub fn gcd_multiple<T, U, I>(nums: I) -> T
where
    T: PrimInt + Unsigned + ConstZero,
    U: Borrow<T>,
    I: IntoIterator<Item = U>,
{
    let mut nums = nums.into_iter();
    let n1 = *nums
        .next()
        .expect("There must be at least 2 numbers.")
        .borrow();
    let n2 = *nums
        .next()
        .expect("There must be at least 2 numbers.")
        .borrow();
    let mut result = gcd(n1, n2);
    for n in nums {
        result = gcd(result, *n.borrow());
    }
    result
}

/// Checks whether an unsigned integer is a palindrome.
/// # Arguments
/// * `num` - The unsigned integer to check.
/// * `radix` - The radix to use for checking.
/// # Returns
/// * `bool` - Whether the number is a palindrome.
/// # Example
/// ```
/// use peuler::math::is_palindrome;
///
/// // 12321 is a palindrome
/// assert!(is_palindrome(12321u16, 10));
///
/// // 12345 is not a palindrome
/// assert!(!is_palindrome(12345u16, 10));
///
/// // binary 110011 is a palindrome
/// assert!(is_palindrome(0b110011u8, 2));
/// ```
pub fn is_palindrome<T>(num: T, radix: u8) -> bool
where
    T: PrimInt + Unsigned + ConstZero,
{
    num == reverse(num, radix)
}

/// Checks if two numbers are permutations of each other.
/// # Arguments
/// * `n` - The first number.
/// * `m` - The second number.
/// * `radix` - The radix of the numbers.
/// # Returns
/// * `bool` - Whether the numbers are permutations of each other.
/// # Example
/// ```
/// use peuler::math::is_permutation;
/// // 123 and 321 are permutations
/// assert!(is_permutation(123, 321, 10));
/// // 123 and 3210 are not permutations
/// assert!(!is_permutation(123, 3210, 10));
/// // binary 1101 and 1011 are permutations
/// assert!(is_permutation(0b1101, 0b1011, 2));
/// ```
pub fn is_permutation(n: u64, m: u64, radix: u8) -> bool {
    let mut seen_digits = [0_i8; 256];

    for digit in digits(n, radix) {
        seen_digits[digit as usize] += 1;
    }
    for digit in digits(m, radix) {
        seen_digits[digit as usize] -= 1;
    }

    seen_digits.iter().all(|&count| count == 0)
}

/// Calculate the integer square root.
/// Slower than casting to f64 and using .sqrt().floor().
/// To be used with big numbers which would lose precision if cast to f64.
/// Uses Newton's method.
/// # Arguments
/// * `n` - The number to find the integer square root of.
/// # Returns
/// * `u64` - The integer square root.
/// # Example
/// ```
/// use peuler::math::isqrt;
/// // isqrt of 12 is 3
/// assert_eq!(isqrt(12), 3);
/// ```
pub fn isqrt(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        let mut x0 = 2_u64.pow((n.ilog2() >> 1) + 1);
        let mut x1 = (x0 + n / x0) >> 1;
        while x1 < x0 {
            x0 = x1;
            x1 = (x0 + n / x0) >> 1;
        }
        x0
    }
}

/// Calculate the integer square root of an u128 number.
/// Same as isqrt, but for u128.
/// # Arguments
/// * `n` - The number to find the integer square root of.
/// # Returns
/// * `u128` - The integer square root.
/// # Example
/// ```
/// use peuler::math::isqrt_128;
/// // isqrt of 12 is 3
/// assert_eq!(isqrt_128(12), 3);
/// ```
pub fn isqrt_128(n: u128) -> u128 {
    if n <= 1 {
        n
    } else {
        let mut x0 = 2_u128.pow((n.ilog2() >> 1) + 1);
        let mut x1 = (x0 + n / x0) >> 1;
        while x1 < x0 {
            x0 = x1;
            x1 = (x0 + n / x0) >> 1;
        }
        x0
    }
}

/// Finds the least common multiple of two integers.
/// # Arguments
/// * `n1` - The first integer.
/// * `n2` - The second integer.
/// # Returns
/// * The least common multiple.
/// # Example
/// ```
/// use peuler::math::lcm;
///
/// // lcm of 12 and 18 is 36
/// assert_eq!(lcm(12u8, 18u8), 36);
/// ```
pub fn lcm<T>(n1: T, n2: T) -> T
where
    T: PrimInt + Unsigned + ConstZero,
{
    (n1 / gcd(n1, n2)) * n2
}

/// Finds the least common multiple of multiple integers.
/// # Arguments
/// * `nums` - The integers to find the least common multiple of.
/// # Returns
/// * The least common multiple.
/// # Panics
/// If there are less than 2 numbers.
/// # Example
/// ```
/// use peuler::math::lcm_multiple;
///
/// // lcm of 12, 18 and 24 is 72
/// assert_eq!(lcm_multiple([12u8, 18u8, 24u8]), 72);
/// ```
pub fn lcm_multiple<T, U, I>(nums: I) -> T
where
    T: PrimInt + Unsigned + ConstZero,
    U: Borrow<T>,
    I: IntoIterator<Item = U>,
{
    let mut nums = nums.into_iter();
    let n1 = *nums
        .next()
        .expect("There must be at least 2 numbers.")
        .borrow();
    let n2 = *nums
        .next()
        .expect("There must be at least 2 numbers.")
        .borrow();
    let mut result = lcm(n1, n2);
    for n in nums {
        result = lcm(result, *n.borrow());
    }
    result
}

/// Calculates the zero of a function using Newton's method.
/// Note that this will run forever if the function does not converge to a zero.
/// # Arguments
/// * `x0` - The initial guess.
/// * `precision` - The precision of the answer (the error will be less than this).
/// * `function` - The function to find the zero of.
/// * `derivative` - The derivative of the function.
/// # Returns
/// * `f64` - The zero of the function.
/// # Panics
/// If the derivative is 0 at some of the evaluated points.
/// # Example
/// ```
/// use peuler::math::newtons_method;
/// // zero of x^2 - 2 = 0 is sqrt(2)
/// let x0 = 1.0;
/// let precision = 1e-10;
/// let function = |x| x * x - 2.0;
/// let derivative = |x| 2.0 * x;
/// assert!((newtons_method(x0, precision, function, derivative) - 2.0_f64.sqrt()).abs() < precision);
/// ```
pub fn newtons_method<F, D>(x0: f64, precision: f64, function: F, derivative: D) -> f64
where
    F: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    let mut x = x0;
    let mut prev_x = f64::NEG_INFINITY;

    while (x - prev_x).abs() > precision {
        prev_x = x;
        x = prev_x - function(prev_x) / derivative(prev_x);
    }

    x
}

/// Finds the number of divisors of a number.
/// # Arguments
/// * `n` - The number to find the number of divisors of.
/// # Returns
/// * `u64` - The number of divisors of the number.
/// # Example
/// ```
/// use peuler::math::num_of_divisors;
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
/// use peuler::math::num_of_divisors_1_to_n;
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
/// use peuler::math::num_of_proper_divisors;
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
/// use peuler::math::num_of_proper_divisors_1_to_n;
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

/// Calculates multiplicative order.
/// (smallest positive integer k such that a^k ≡ 1 (mod n)).
/// a and n must be coprime.
/// # Arguments
/// * `a` - The base.
/// * `n` - The modulus.
/// # Returns
/// * `u64` - The multiplicative order.
/// # Panics
/// If a and n are not coprime.
/// # Example
/// ```
/// use peuler::math::ord;
/// // ord(3, 7) = 6
/// assert_eq!(ord(3, 7), 6);
/// ```
pub fn ord(a: u64, n: u64) -> u64 {
    // a^k ≡ 1 (mod n)
    // a^k (mod n) = ((a^(k-1) (mod n)) * a) (mod n)
    // example: 8^2 mod 7 = ((8 mod 7) * 8) mod 7
    // k <= n - 1 (Fermat's little theorem)

    let mut result = 1;
    for k in 1..n {
        result = (result * a) % n;
        if result == 1 {
            return k;
        }
    }

    // since a and n are coprime, multiplicative order must exist
    panic!("Multiplicative order not found (a and n must be coprime).");
}

/// Calculates the partition function
/// (number of ways a number can be written as a sum of positive integers).
/// Uses the recurrence relation p(n) = Σ(k=1, n)(-1)^(k+1) * (p(n - k(3k - 1) / 2) + p(n - k(3k + 1) / 2)).
/// # Arguments
/// * `n` - The number to find the number of partitions of.
/// # Returns
/// * `u64` - The number of partitions of the number.
/// # Example
/// ```
/// use peuler::math::partition_p;
/// // Partitions of 5: {5}, {4, 1}, {3, 2}, {3, 1, 1}, {2, 2, 1}, {2, 1, 1, 1}, {1, 1, 1, 1, 1} == 7
/// assert_eq!(partition_p(5u8), 7);
/// ```
pub fn partition_p<T>(n: T) -> u64
where
    T: PrimInt + Unsigned + ToPrimitive,
{
    // since calculating p(n) also requires calculating p of every number less than n,
    // so we just calculate all values and get the value of p(n) from the vector (last value)
    partition_p_1_to_n(n).pop().unwrap()
}

/// Calculates the partition function for numbers from 1 to n
/// (number of ways a number can be written as a sum of positive integers).
/// Uses the recurrence relation p(n) = Σ(k=1, n)(-1)^(k+1) * (p(n - k(3k - 1) / 2) + p(n - k(3k + 1) / 2)).
/// # Arguments
/// * `n` - The number to find the number of partitions of.
/// # Returns
/// * `Vec<u64>` - The number of partitions of numbers from 0 to n. Index represents the number.
/// # Example
/// ```
/// use peuler::math::partition_p_1_to_n;
/// assert_eq!(partition_p_1_to_n(10u8), vec![1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42]);
/// ```
pub fn partition_p_1_to_n<T>(n: T) -> Vec<u64>
where
    T: PrimInt + Unsigned + ToPrimitive,
{
    // get n as usize
    let n = n.to_usize().expect("Number too large.");

    // if n is 0, return 1
    if n == 0 {
        return vec![1];
    }

    let mut partitions = Vec::with_capacity(n + 1);
    partitions.push(1);
    partitions.push(1);

    while partitions.len() <= n {
        // calculate next value and add it to vector

        let curr_n = partitions.len();
        let mut next_val = 0;
        for k in 1..=curr_n {
            let left_value = match curr_n.checked_sub((k * (3 * k - 1)) >> 1) {
                Some(ind) => partitions[ind],
                None => break, // larger of the indices is below zero, so any larger k will only be 0, we can break
            };
            let right_value = match curr_n.checked_sub((k * (3 * k + 1)) >> 1) {
                Some(ind) => partitions[ind],
                None => 0,
            };
            let value = left_value + right_value;

            if k % 2 == 0 {
                next_val -= value;
            } else {
                next_val += value;
            }
        }

        // push the newly calculated value to the vector
        partitions.push(next_val);
    }

    // return the partitions vector
    partitions
}

/// Calculates the number of prime partitions
/// (a number of ways a number can be written as a sum of primes).
/// # Arguments
/// * `n` - The number to find the number of prime partitions of.
/// # Returns
/// * `u64` - The number of prime partitions of the number.
/// # Example
/// ```
/// use peuler::math::partition_prime;
/// // Prime partitions of 7: {7}, {5, 2}, {3, 2, 2}
/// assert_eq!(partition_prime(7u8), 3);
/// ```
pub fn partition_prime<T>(n: T) -> u64
where
    T: PrimInt + Unsigned + ToPrimitive,
{
    // since calculating p(n) also requires calculating p of every number less than n,
    // so we just calculate all values and get the value of p(n) from the vector (last value)
    partition_prime_1_to_n(n).pop().unwrap()
}

/// Calculates the number of prime partitions for numbers from 1 to n.
/// Prime partitions is a number of ways a number can be written as a sum of primes.
/// # Arguments
/// * `n` - The number to find the number of prime partitions of.
/// # Returns
/// * `Vec<u64>` - The number of prime partitions of numbers from 0 to n. Index represents the number.
/// # Example
/// ```
/// use peuler::math::partition_prime_1_to_n;
/// assert_eq!(partition_prime_1_to_n(10u8), vec![1, 0, 1, 1, 1, 2, 2, 3, 3, 4, 5]);
/// ```
pub fn partition_prime_1_to_n<T>(n: T) -> Vec<u64>
where
    T: PrimInt + Unsigned + ToPrimitive,
{
    let n = n.to_usize().expect("Number too large.");
    let primes = sieve_of_eratosthenes(n as u64);

    let mut dp = vec![0; n + 1];

    // 0 can be represented in 1 way = {} (1 can't be represented as a sum of primes so dp[1] stays 0)
    dp[0] = 1;

    for prime in primes {
        for i in (prime as usize)..=n {
            dp[i] += dp[i - prime as usize];
        }
    }

    dp
}

/// Calculate the Euler's totient function.
/// Finds the number of positive integers less than n that are coprime to n.
/// # Arguments
/// * `n` - The number to find the Euler's totient function of.
/// # Returns
/// * `u64` - The Euler's totient function of the number.
/// # Example
/// ```
/// use peuler::math::phi;
/// // phi(0) = 0, phi(1) = 1, phi(2) = 1, phi(3) = 2, phi(4) = 2, phi(5) = 4
/// assert_eq!(phi(0), 0);
/// assert_eq!(phi(1), 1);
/// assert_eq!(phi(2), 1);
/// assert_eq!(phi(3), 2);
/// assert_eq!(phi(4), 2);
/// assert_eq!(phi(5), 4);
/// ```
pub fn phi(n: u64) -> u64 {
    let mut result = n;
    distinct_prime_factors(n)
        .map(|(factor, _)| factor)
        .for_each(|factor| {
            result -= result / factor;
        });
    result
}

/// Calculate the Euler's totient function for numbers from 1 to n.
/// Returns a vector of the results, indices represent the number.
/// # Arguments
/// * `n` - The number to find the Euler's totient function of.
/// # Returns
/// * `Vec<u64>` - The Euler's totient function of numbers from 0 to n. Index represents the number.
/// # Example
/// ```
/// use peuler::math::phi_1_to_n;
/// // phi(0) = 0, phi(1) = 1, phi(2) = 1, phi(3) = 2, phi(4) = 2, phi(5) = 4
/// assert_eq!(phi_1_to_n(5), vec![0, 1, 1, 2, 2, 4]);
/// ```
pub fn phi_1_to_n(n: u64) -> Vec<u64> {
    let mut phi_values = (0..=n).collect_vec();

    for i in 2..=n {
        if phi_values[i as usize] == i {
            for j in (i..=n).step_by(i as usize) {
                phi_values[j as usize] -= phi_values[j as usize] / i;
            }
        }
    }

    phi_values
}

/// Reverses an unsigned integer.
/// # Arguments
/// * `num` - The unsigned integer to reverse.
/// * `radix` - The radix to use for reversing the integer.
/// # Returns
/// * The reversed integer.
/// # Example
/// ```
/// use peuler::math::reverse;
/// // 123 -> 321
/// assert_eq!(reverse(123u16, 10), 321);
/// // 0 -> 0
/// assert_eq!(reverse(0u8, 10), 0);
/// // binary 1101 -> 1011
/// assert_eq!(reverse(0b1101u8, 2), 0b1011);
/// ```
pub fn reverse<T>(mut num: T, radix: u8) -> T
where
    T: PrimInt + Unsigned + ConstZero,
{
    let radix = T::from(radix).unwrap();
    let mut new_num = T::ZERO;
    while num > T::ZERO {
        new_num = new_num * radix + num % radix;
        num = num / radix;
    }
    new_num
}

/// Finds the sum of the squares of the first n even natural numbers.
/// # Arguments
/// * `n` - The number of even natural numbers to sum.
/// # Returns
/// * The sum of the squares of the first n even natural numbers.
/// # Example
/// ```
/// use peuler::math::sum_n_even_squares;
/// // 2^2 + 4^2 + 6^2 + 8^2 + 10^2 = 220
/// assert_eq!(sum_n_even_squares(5u16), 220);
/// ```
pub fn sum_n_even_squares<T>(n: T) -> T
where
    T: PrimInt + Unsigned + ConstOne + NumCast,
{
    let two = T::from(2).unwrap();
    two * n * (n + T::ONE) * (two * n + T::ONE) / T::from(3).unwrap()
}

/// Finds the sum of the squares of the first n odd natural numbers.
/// # Arguments
/// * `n` - The number of odd natural numbers to sum.
/// # Returns
/// * The sum of the squares of the first n odd natural numbers.
/// # Example
/// ```
/// use peuler::math::sum_n_odd_squares;
/// // 1^2 + 3^2 + 5^2 + 7^2 + 9^2 = 165
/// assert_eq!(sum_n_odd_squares(5u16), 165);
/// ```
pub fn sum_n_odd_squares<T>(n: T) -> T
where
    T: PrimInt + Unsigned + ConstZero + ConstOne + NumCast,
{
    let two = T::from(2).unwrap();
    if n == T::ZERO {
        T::ZERO
    } else {
        n * (two * n + T::ONE) * (two * n - T::ONE) / T::from(3).unwrap()
    }
}

/// Finds the sum of the divisors of a number.
/// # Arguments
/// * `n` - The number to find the sum of the divisors of.
/// # Returns
/// * `u64` - The sum of the divisors of the number.
/// # Example
/// ```
/// use peuler::math::sum_of_divisors;
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
/// use peuler::math::sum_of_divisors_1_to_n;
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
/// use peuler::math::sum_of_proper_divisors;
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
/// use peuler::math::sum_of_proper_divisors_1_to_n;
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
