//! Mathematical functions.

use malachite::num::basic::traits::{One, Zero};
use malachite::{Integer, Rational};
use num_traits::{ConstOne, ConstZero, NumCast, PrimInt, Unsigned};
use std::borrow::Borrow;
use std::collections::HashSet;
use std::iter;
use std::mem;
use std::ops::{Add, Mul, Sub};
use std::sync::{LazyLock, Mutex};

/// Inverse of the prime-counting function.
/// Estimates the number for which the prime-counting function is approximately n.
/// Uses the inverse of the prime number theorem.
/// Answer is approximated using Newton's method.
/// It is exact for n <= 3 and guaranteed to be an overestimate for n > 3.
/// # Arguments
/// * `n` - The number to estimate the inverse of the prime-counting function for.
/// # Returns
/// * `f64` - The estimated inverse of the prime-counting function for n.
pub fn apcf(n: u64) -> f64 {
    match n {
        0 => 0.0,
        1 => 2.0,
        2 => 3.0,
        3 => 5.0,
        _ => {
            let a = n as f64;
            let newtons = |x: f64| (a * x * (x.ln() - 1.0)) / (x - a);
            let mut prev_x = f64::NEG_INFINITY;
            let mut x = a + 1.0;
            while (x - prev_x).abs() > 1e-10 {
                prev_x = x;
                x = newtons(x);
            }
            x
        }
    }
}

/// Returns the iterator of the Collatz sequence starting at a number.
pub fn collatz_seq(num: u64) -> impl Iterator<Item = u64> {
    let mut current = num;
    iter::from_fn(move || {
        if current == 1 {
            return None;
        }
        if current % 2 == 0 {
            current >>= 1;
        } else {
            current = 3 * current + 1;
        }
        Some(current)
    })
}

/// Represents a continued fraction.
pub struct ContinuedFraction {
    non_periodic: Vec<i64>,
    periodic: Option<Vec<i64>>,
}
impl ContinuedFraction {
    /// Creates a new continued fraction.
    pub fn new(non_periodic: Vec<i64>, periodic: Option<Vec<i64>>) -> Self {
        Self { non_periodic, periodic }
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

        Self { non_periodic, periodic }
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
    pub fn convergents(&self) -> impl Iterator<Item = Rational> + '_ {
        let mut prev_num = Integer::ZERO;
        let mut prev_den = Integer::ONE;
        let mut num = Integer::ONE;
        let mut den = Integer::ZERO;
        let mut values = self.non_periodic.iter().chain(self.periodic.iter().flat_map(|v| v.iter().cycle()));

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
/// use project_euler::shared::math::digits;
///
/// assert_eq!(digits(123, 10).collect::<Vec<u8>>(), vec![1, 2, 3]);
/// assert_eq!(digits(123, 10).rev().collect::<Vec<u8>>(), vec![3, 2, 1]);
/// assert_eq!(digits(0, 10).len(), 0);
/// assert_eq!(digits(123, 10).rev().len(), 3);
/// ```
pub fn digits(n: u64, radix: u64) -> DigitsIter {
    DigitsIter::new(n, radix)
}
pub struct DigitsIter {
    num: u64,
    radix: u64,
    front_weight: u64,
    length: usize,
}
impl DigitsIter {
    fn new(num: u64, radix: u64) -> Self {
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

/// Calculates the factorial of a number.
/// # Arguments
/// * `n` - The number to find the factorial of.
/// # Returns
/// * The factorial of the number.
/// # Example
/// ```
/// use project_euler::shared::math::factorial;
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

/// Finds the greatest common divisor of two numbers.
/// Uses the Euclidean algorithm.
/// # Arguments
/// * `num1` - The first number.
/// * `num2` - The second number.
/// # Returns
/// * The greatest common divisor.
/// # Example
/// ```
/// use project_euler::shared::math::gcd;
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
/// use project_euler::shared::math::gcd_multiple;
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
    let n1 = *nums.next().expect("There must be at least 2 numbers.").borrow();
    let n2 = *nums.next().expect("There must be at least 2 numbers.").borrow();
    let mut result = gcd(n1, n2);
    for n in nums {
        result = gcd(result, *n.borrow());
    }
    result
}

/// Checks if a number is a palindrome.
/// # Arguments
/// * `num` - The number to check.
/// # Returns
/// * `bool` - Whether the number is a palindrome.
/// # Example
/// ```
/// use project_euler::shared::math::is_palindrome;
/// // 12321 is a palindrome
/// assert!(is_palindrome(12321u16));
/// // 12345 is not a palindrome
/// assert!(!is_palindrome(12345u16));
/// ```
pub fn is_palindrome<T>(num: T) -> bool
where
    T: PrimInt + Unsigned + ConstZero + NumCast,
{
    num == reverse(num)
}

/// Checks if two numbers are permutations of each other.
pub fn is_permutation(n: u64, m: u64) -> bool {
    let mut n_digits = [0_u8; 10];
    let mut m_digits = [0_u8; 10];

    for digit in digits(n, 10) {
        n_digits[digit as usize] += 1;
    }
    for digit in digits(m, 10) {
        m_digits[digit as usize] += 1;
    }

    n_digits == m_digits
}

/// Checks if a number is prime.
/// # Arguments
/// * `num` - The number to check.
/// # Returns
/// * `bool` - Whether the number is prime.
/// * `u64` - The smallest divisor if the number is not prime, otherwise 1.
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

/// Calculate the integer square root.
/// Slower than casting to f64 and using .sqrt().floor().
/// To be used with big numbers which would lose precision if cast to f64.
/// Uses Newton's method.
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

/// Creates an integer from digits.
/// Digits can be any type that implements [IntoIterator].
/// # Arguments
/// * `digits` - The type that implements [IntoIterator] and contains digits.
/// # Returns
/// * `u64` - The integer.
/// # Example
/// ```
/// use project_euler::shared::math::digits_to_int;
/// // 123 -> 123
/// assert_eq!(digits_to_int([1u8, 2u8, 3u8], 10), 123);
/// ```
pub fn digits_to_int<T, U>(digits: T, radix: u64) -> u64
where
    T: IntoIterator<Item = U>,
    U: Borrow<u8>,
{
    let mut result = 0;
    for digit in digits {
        result *= radix;
        result += *digit.borrow() as u64;
    }
    result
}

/// Finds the least common multiple of two numbers.
/// # Arguments
/// * `num1` - The first number.
/// * `num2` - The second number.
/// # Returns
/// * The least common multiple.
/// # Example
/// ```
/// use project_euler::shared::math::lcm;
/// // lcm of 12 and 18 is 36
/// assert_eq!(lcm(12u8, 18u8), 36);
/// ```
pub fn lcm<T>(num1: T, num2: T) -> T
where
    T: PrimInt + Unsigned + ConstZero,
{
    (num1 / gcd(num1, num2)) * num2
}

/// Finds the least common multiple of multiple numbers.
/// # Arguments
/// * `nums` - The numbers.
/// # Returns
/// * The least common multiple.
/// # Panics
/// If there are less than 2 numbers.
/// # Example
/// ```
/// use project_euler::shared::math::lcm_multiple;
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
    let n1 = *nums.next().expect("There must be at least 2 numbers.").borrow();
    let n2 = *nums.next().expect("There must be at least 2 numbers.").borrow();
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
pub fn num_of_divisors(n: u64) -> u64 {
    // let n be a natural number
    // we can factorise n
    // n = p1^a1 * p2^a2 * p3^a3 * ... * pn^an
    // (where p1, p2, p3, ..., pn are prime numbers)
    // a function d(n) returns the number of divisors of n
    // obviously, d(n) = (a1 + 1) * (a2 + 1) * (a3 + 1) * ... * (an + 1)

    prime_factors(n).into_iter().map(|(_, a)| a + 1).product()
}

/// Finds the number of divisors of numbers from 1 to n.
/// # Arguments
/// * `n` - The number to find the number of divisors of.
/// # Returns
/// * `Vec<u64>` - The number of divisors of numbers from 1 to n. Index 0 is unused (set to 0), other indices represent the number.
pub fn num_of_divisors_1_to_n(n: u64) -> Vec<u64> {
    let mut divisors = vec![1; (n + 1) as usize];
    divisors[0] = 0;
    for i in 2..=n {
        for j in (i..=n).step_by(i as usize) {
            divisors[j as usize] += 1;
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
pub fn num_of_proper_divisors(n: u64) -> u64 {
    num_of_divisors(n) - 1
}

/// Finds the number of proper divisors of numbers from 1 to n.
/// Proper divisors are all divisors of a number except the number itself.
/// # Arguments
/// * `n` - The number to find the number of proper divisors of.
/// # Returns
/// * `Vec<u64>` - The number of proper divisors of numbers from 1 to n. Index 0 is unused (set to 0), other indices represent the number.
pub fn num_of_proper_divisors_1_to_n(n: u64) -> Vec<u64> {
    let mut divisors = vec![0; (n + 1) as usize];
    for i in 2..=n {
        for j in (i..=n).step_by(i as usize) {
            divisors[j as usize] += 1;
        }
    }
    divisors
}

/// Calculates multiplicative order.
/// Finds the smallest positive integer k such that a^k ≡ 1 (mod n).
/// a and n must be coprime.
/// # Arguments
/// * `a` - The base.
/// * `n` - The modulus.
/// # Returns
/// * `u64` - The multiplicative order.
/// # Panics
/// If a and n are not coprime.
pub fn ord(a: u64, n: u64) -> u64 {
    assert_eq!(gcd(a, n), 1, "a and n must be coprime.");

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
    unreachable!("Multiplicative order not found.");
}

/// Calculates the partition function.
/// Finds the number of ways a number can be written as a sum of positive integers.
/// Uses the recurrence relation p(n) = Σ(k=1, n)(-1)^(k+1) * (p(n - k(3k - 1) / 2) + p(n - k(3k + 1) / 2)).
/// Uses memoization to speed up the calculation.
/// # Arguments
/// * `n` - The number to find the number of partitions of.
/// # Returns
/// * `u64` - The number of partitions of the number.
/// # Example
/// Partitions of 5: {5}, {4, 1}, {3, 2}, {3, 1, 1}, {2, 2, 1}, {2, 1, 1, 1}, {1, 1, 1, 1, 1}
///
/// partition_p(5) = 7
pub fn partition_p(n: u64) -> u64 {
    // get n as usize
    let n = usize::try_from(n).expect("Number too large.");

    // since calculating p(n) also requires calculating p of every number less than n
    // we can just calculate all values and store them in a vector

    // memoization
    static CACHE_VEC: LazyLock<Mutex<Vec<u64>>> = LazyLock::new(|| Mutex::new(vec![1_u64, 1_u64]));
    let mut cache = CACHE_VEC.lock().unwrap();

    // if n is already in the cache vector, then return the value
    if let Some(&value) = cache.get(n) {
        return value;
    }

    while cache.len() <= n {
        // calculate next value and add it to vector

        let curr_n = cache.len();
        let mut next_val = 0;
        for k in 1..=curr_n {
            let left_value = match curr_n.checked_sub((k * (3 * k - 1)) >> 1) {
                Some(ind) => cache[ind],
                None => break, // larger of the indices is below zero, so any larger k will only be 0, we can break
            };
            let right_value = match curr_n.checked_sub((k * (3 * k + 1)) >> 1) {
                Some(ind) => cache[ind],
                None => 0,
            };
            let value = left_value + right_value;

            if k % 2 == 0 {
                next_val -= value;
            } else {
                next_val += value;
            }
        }

        // push the newly calculated value to the cache vector
        cache.push(next_val);
    }

    // return the value
    cache[n]
}

/// Calculates the number of prime partitions.
/// Finds the number of ways a number can be written as a sum of primes.
/// # Arguments
/// * `n` - The number to find the number of prime partitions of.
/// # Returns
/// * `u64` - The number of prime partitions of the number.
/// # Example
/// Prime partitions of 7: {7}, {5, 2}, {3, 2, 2}
///
/// partition_prime(7) = 3
pub fn partition_prime(n: u64) -> u64 {
    let n = usize::try_from(n).expect("Number too large.");
    let primes = sieve_of_eratosthenes(n as u64);

    let mut dp = vec![0; n + 1];

    // 0 can be represented in 1 way = {} (1 can't be represented as a sum of primes so dp[1] stays 0)
    dp[0] = 1;

    for prime in primes {
        for i in (prime as usize)..=n {
            dp[i] += dp[i - prime as usize];
        }
    }

    dp[n]
}

/// Simple prime-counting function.
/// Estimates the number of primes less than or equal to x.
/// Uses the prime number theorem which states that the number of primes less than or equal to x is approximately x / ln(x).
/// It is exact for x <= 10 and guaranteed to be an underestimate for x > 10.
/// # Arguments
/// * `x` - The number to estimate the number of primes less than or equal to.
/// # Returns
/// * `f64` - The estimated number of primes less than or equal to x.
pub fn pcf(x: u64) -> f64 {
    match x {
        0..=1 => 0.0,
        2 => 1.0,
        3..=4 => 2.0,
        5..=6 => 3.0,
        7..=10 => 4.0,
        _ => x as f64 / (x as f64).ln(),
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
pub fn pcf_exact(x: u64) -> u64 {
    sieve_of_eratosthenes(x).len() as u64
}

/// Calculate the Euler's totient function.
pub fn phi(n: u64) -> u64 {
    let mut result = n;
    let mut curr_fact = 0;
    for fact in prime_factors_iter(n) {
        if curr_fact != fact {
            curr_fact = fact;
            result -= result / fact;
        }
    }
    result
}

/// Calculate the Euler's totient function for numbers from 1 to n.
/// Returns a vector of the results, index 0 is unused (set to 0), other indices represent the number.
pub fn phi_1_to_n(n: u64) -> Vec<u64> {
    let mut phi_values: Vec<u64> = (0..=n).collect();

    for i in 2..=n {
        if phi_values[i as usize] == i {
            for j in (i..=n).step_by(i as usize) {
                phi_values[j as usize] -= phi_values[j as usize] / i;
            }
        }
    }

    phi_values
}

/// A point in N-dimensional space.
/// # Example
/// ```
/// use project_euler::shared::math::Point;
/// let point = Point::new([1.0, 2.0, 3.0]);
/// assert_eq!(point.coords, [1.0, 2.0, 3.0]);
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point<const N: usize> {
    pub coords: [f64; N],
}
impl<const N: usize> Point<N> {
    /// Creates a new point.
    /// # Arguments
    /// * `coords` - The coordinates of the point.
    /// # Returns
    /// * `Point` - The new point.
    pub fn new(coords: [f64; N]) -> Self {
        Self { coords }
    }
}

/// Finds the prime factors of a number.
/// If the number is 0 or 1, then an empty vector is returned.
/// # Arguments
/// * `x` - The number to find the prime factors of.
/// # Returns
/// * `Vec<(u64, u64)>` - The prime factors of the number. In the form (prime_factor, power).
pub fn prime_factors(mut x: u64) -> Vec<(u64, u64)> {
    // calculate primes that are less than or equal to the square root of x
    // these are the only possible prime factors
    let prime_table = sieve_of_eratosthenes((x as f64).sqrt().floor() as u64);
    let mut factors = Vec::new();

    // for every prime factor divide x by that prime factor until it is no longer divisible by that prime factor
    // if x becomes 1 then we have found all prime factors and don't need to check further
    for prime_fact in prime_table {
        let mut fact_info = (prime_fact, 0);

        while x % prime_fact == 0 {
            fact_info.1 += 1;
            x /= prime_fact;
        }

        if fact_info.1 > 0 {
            factors.push(fact_info);
        }

        if x == 1 {
            break;
        }
    }

    // if x is not 1 here, then there are two options
    // 1. x is prime -> we add it to the list of prime factors
    // 2. x is zero -> we do nothing
    if x != 1 && x != 0 {
        factors.push((x, 1));
    }

    factors
}

/// Returns the prime factors as an iterator.
/// If the prime factor appears multiple times, then it will appear multiple times in the iterator.
/// Factors are returned in ascending order.
/// Makes no memory allocations, therefore, it is better for small numbers.
/// For big numbers, prime_factors should be faster because it uses prime sieve.
pub fn prime_factors_iter(mut x: u64) -> impl Iterator<Item = u64> {
    let mut factor = 2;
    iter::from_fn(move || {
        while factor <= x {
            if x % factor == 0 {
                x /= factor;
                return Some(factor);
            }
            match factor {
                2 => factor += 1,
                _ => factor += 2,
            };
        }
        None
    })
}

/// Reverses a number.
/// # Arguments
/// * `num` - The number to reverse.
/// # Returns
/// * The reversed number.
/// # Example
/// ```
/// use project_euler::shared::math::reverse;
/// // 123 -> 321
/// assert_eq!(reverse(123u16), 321);
/// // 0 -> 0
/// assert_eq!(reverse(0u8), 0);
/// ```
pub fn reverse<T>(mut num: T) -> T
where
    T: PrimInt + Unsigned + ConstZero + NumCast,
{
    let ten = T::from(10).unwrap();
    let mut new_num = T::ZERO;
    while num > T::ZERO {
        new_num = new_num * ten + num % ten;
        num = num / ten;
    }
    new_num
}

/// The sieve of Eratosthenes.
/// Finds all primes less than or equal to n.
/// # Arguments
/// * `n` - The number to find all primes less than or equal to.
/// # Returns
/// * `Vec<u64>` - All primes less than or equal to n.
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
            results.extend(sieve.into_iter().enumerate().filter_map(|(i, prime)| if prime { Some(ind_to_val(i)) } else { None }));

            // return results
            results
        }
    }
}

/// Finds the sum of the first n natural numbers.
/// # Arguments
/// * `n` - The number of natural numbers to sum.
/// # Returns
/// * The sum of the first n natural numbers.
/// # Example
/// ```
/// use project_euler::shared::math::sum_n;
/// // 1 + 2 + 3 + 4 + 5 = 15
/// assert_eq!(sum_n(5u8), 15);
/// ```
pub fn sum_n<T>(n: T) -> T
where
    T: PrimInt + Unsigned + ConstOne + NumCast,
{
    let two = T::from(2).unwrap();
    n * (n + T::ONE) / two
}

/// Finds the sum of the first n even natural numbers.
/// # Arguments
/// * `n` - The number of even natural numbers to sum.
/// # Returns
/// * The sum of the first n even natural numbers.
/// # Example
/// ```
/// use project_euler::shared::math::sum_n_even;
/// // 2 + 4 + 6 + 8 + 10 = 30
/// assert_eq!(sum_n_even(5u8), 30);
/// ```
pub fn sum_n_even<T>(n: T) -> T
where
    T: PrimInt + Unsigned + ConstOne,
{
    n * (n + T::ONE)
}

/// Finds the sum of the squares of the first n even natural numbers.
/// # Arguments
/// * `n` - The number of even natural numbers to sum.
/// # Returns
/// * The sum of the squares of the first n even natural numbers.
/// # Example
/// ```
/// use project_euler::shared::math::sum_n_even_squares;
/// // 2^2 + 4^2 + 6^2 + 8^2 + 10^2 = 220
/// assert_eq!(sum_n_even_squares(5u16), 220);
/// ```
pub fn sum_n_even_squares<T>(n: T) -> T
where
    T: PrimInt + Unsigned + ConstOne + NumCast,
{
    let two = T::from(2).unwrap();
    let three = T::from(3).unwrap();
    two * n * (n + T::ONE) * (two * n + T::ONE) / three
}

/// Finds the sum of the first n odd natural numbers.
/// # Arguments
/// * `n` - The number of odd natural numbers to sum.
/// # Returns
/// * The sum of the first n odd natural numbers.
/// # Example
/// ```
/// use project_euler::shared::math::sum_n_odd;
/// // 1 + 3 + 5 + 7 + 9 = 25
/// assert_eq!(sum_n_odd(5u8), 25);
/// ```
pub fn sum_n_odd<T>(n: T) -> T
where
    T: PrimInt + Unsigned,
{
    n * n
}

/// Finds the sum of the squares of the first n odd natural numbers.
/// # Arguments
/// * `n` - The number of odd natural numbers to sum.
/// # Returns
/// * The sum of the squares of the first n odd natural numbers.
/// # Example
/// ```
/// use project_euler::shared::math::sum_n_odd_squares;
/// // 1^2 + 3^2 + 5^2 + 7^2 + 9^2 = 165
/// assert_eq!(sum_n_odd_squares(5u16), 165);
/// ```
pub fn sum_n_odd_squares<T>(n: T) -> T
where
    T: PrimInt + Unsigned + ConstZero + ConstOne + NumCast,
{
    let two = T::from(2).unwrap();
    let three = T::from(3).unwrap();

    if n == T::ZERO {
        T::ZERO
    } else {
        n * (two * n + T::ONE) * (two * n - T::ONE) / three
    }
}

/// Finds the sum of the squares of the first n natural numbers.
/// # Arguments
/// * `n` - The number of natural numbers to sum.
/// # Returns
/// * The sum of the squares of the first n natural numbers.
/// # Example
/// ```
/// use project_euler::shared::math::sum_n_squares;
/// // 1^2 + 2^2 + 3^2 + 4^2 + 5^2 = 55
/// assert_eq!(sum_n_squares(5u16), 55);
/// ```
pub fn sum_n_squares<T>(n: T) -> T
where
    T: PrimInt + Unsigned + ConstOne + NumCast,
{
    let two = T::from(2).unwrap();
    let six = T::from(6).unwrap();
    n * (n + T::ONE) * (two * n + T::ONE) / six
}

/// Finds the sum of the divisors of a number.
/// # Arguments
/// * `n` - The number to find the sum of the divisors of.
/// # Returns
/// * `u64` - The sum of the divisors of the number.
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

        prime_factors(n).into_iter().map(|(p, a)| (p.pow(a as u32 + 1) - 1) / (p - 1)).product()
    }
}

/// Finds the sum of the divisors of numbers from 1 to n.
/// # Arguments
/// * `n` - The number to find the sum of the divisors of.
/// # Returns
/// * `Vec<u64>` - The sum of the divisors of numbers from 1 to n. Index 0 is unused (set to 0), other indices represent the number.
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
pub fn sum_of_proper_divisors(n: u64) -> u64 {
    // sum of proper divisors is equal to the sum of all divisors minus the number itself
    sum_of_divisors(n) - n
}

/// Finds the sum of the proper divisors of numbers from 1 to n.
/// Proper divisors are all divisors of a number except the number itself.
/// # Arguments
/// * `n` - The number to find the sum of the proper divisors of.
/// # Returns
/// * `Vec<u64>` - The sum of the proper divisors of numbers from 1 to n. Index 0 is unused (set to 0), other indices represent the number.
pub fn sum_of_proper_divisors_1_to_n(n: u64) -> Vec<u64> {
    let mut divisors = vec![0; (n + 1) as usize];
    for i in 1..=n {
        for j in ((2 * i)..=n).step_by(i as usize) {
            divisors[j as usize] += i;
        }
    }
    divisors
}

/// A vector in N-dimensional space.
/// # Example
/// ```
/// use project_euler::shared::math::Vector;
///
/// let vector = Vector::new([1.0, 2.0, 3.0]);
/// assert_eq!(vector.coords, [1.0, 2.0, 3.0]);
///
/// let quad_vector = 2.0 * vector * 2.0;
/// assert_eq!(quad_vector.coords, [4.0, 8.0, 12.0]);
///
/// let vector2 = Vector::new([4.0, 5.0, 6.0]);
/// let add_vector = vector + vector2;
/// let sub_vector = vector - vector2;
/// assert_eq!(add_vector.coords, [5.0, 7.0, 9.0]);
/// assert_eq!(sub_vector.coords, [-3.0, -3.0, -3.0]);
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<const N: usize> {
    pub coords: [f64; N],
}
impl<const N: usize> Vector<N> {
    /// Creates a new vector.
    /// # Arguments
    /// * `coords` - The coordinates of the vector.
    /// # Returns
    /// * `Vector` - The vector.
    pub fn new(coords: [f64; N]) -> Self {
        Self { coords }
    }
    /// Creates a new vector from 2 points.
    /// # Arguments
    /// * `point1` - The first point.
    /// * `point2` - The second point.
    /// # Returns
    /// * `Vector` - The vector from point1 to point2.
    /// # Example
    /// ```
    /// use project_euler::shared::math::{Point, Vector};
    /// let point1 = Point::new([1.0, 2.0, 3.0]);
    /// let point2 = Point::new([4.0, 5.0, 6.0]);
    /// let vector = Vector::from_points(point1, point2);
    /// assert_eq!(vector.coords, [3.0, 3.0, 3.0]);
    /// ```
    pub fn from_points(point1: Point<N>, point2: Point<N>) -> Self {
        let mut coords = [0.0; N];
        for (i, coord) in coords.iter_mut().enumerate() {
            *coord = point2.coords[i] - point1.coords[i];
        }
        Self { coords }
    }
    /// Calculates the angle between two vectors.
    /// The angle is in radians.
    /// # Arguments
    /// * `other` - The other vector.
    /// # Returns
    /// * `f64` - The angle between the two vectors.
    /// # Panics
    /// If at least one of the vectors is zero.
    /// # Example
    /// ```
    /// use project_euler::shared::math::Vector;
    /// let vector1 = Vector::new([1.0, 0.0]);
    /// let vector2 = Vector::new([0.0, 1.0]);
    /// assert_eq!(vector1.angle_between(&vector2), std::f64::consts::FRAC_PI_2);
    /// ```
    pub fn angle_between(&self, other: &Self) -> f64 {
        (self.dot_product(other) / (self.magnitude() * other.magnitude())).acos()
    }
    /// Calculates the cross product of two vectors.
    /// Only defined for 3D vectors.
    /// # Arguments
    /// * `other` - The other vector.
    /// # Returns
    /// * `Vector` - The cross product of the two vectors.
    /// # Example
    /// ```
    /// use project_euler::shared::math::Vector;
    /// let vector1 = Vector::new([1.0, 0.0, 0.0]);
    /// let vector2 = Vector::new([0.0, 1.0, 0.0]);
    /// assert_eq!(vector1.cross_product(&vector2).coords, [0.0, 0.0, 1.0]);
    /// ```
    pub fn cross_product(&self, other: &Self) -> Self {
        assert_eq!(N, 3, "Cross product is only defined for 3D vectors.");
        let mut coords = [0.0; N];
        coords[0] = self.coords[1] * other.coords[2] - self.coords[2] * other.coords[1];
        coords[1] = self.coords[2] * other.coords[0] - self.coords[0] * other.coords[2];
        coords[2] = self.coords[0] * other.coords[1] - self.coords[1] * other.coords[0];
        Self { coords }
    }
    /// Calculates the dot product of two vectors.
    /// # Arguments
    /// * `other` - The other vector.
    /// # Returns
    /// * `f64` - The dot product of the two vectors.
    /// # Example
    /// ```
    /// use project_euler::shared::math::Vector;
    /// let vector1 = Vector::new([1.0, 2.0, 3.0]);
    /// let vector2 = Vector::new([4.0, 5.0, 6.0]);
    /// assert_eq!(vector1.dot_product(&vector2), 32.0);
    /// ```
    pub fn dot_product(&self, other: &Self) -> f64 {
        self.coords.iter().zip(other.coords.iter()).map(|(&x, &y)| x * y).sum()
    }
    /// Calculates the magnitude of the vector.
    /// # Returns
    /// * `f64` - The magnitude of the vector.
    /// # Example
    /// ```
    /// use project_euler::shared::math::Vector;
    /// let vector = Vector::new([1.0, 2.0, 3.0]);
    /// assert_eq!(vector.magnitude(), 3.7416573867739413);
    /// ```
    pub fn magnitude(&self) -> f64 {
        self.coords.iter().map(|&x| x.powi(2)).sum::<f64>().sqrt()
    }
    /// Calculates the normalized vector.
    /// # Returns
    /// * `Vector` - The normalized vector.
    /// # Panics
    /// If the vector is zero.
    /// # Example
    /// ```
    /// use project_euler::shared::math::Vector;
    /// let vector = Vector::new([1.0, 2.0, 3.0]);
    /// assert_eq!(vector.normalize().coords, [0.2672612419124244, 0.5345224838248488, 0.8017837257372732]);
    /// ```
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        let mut coords = [0.0; N];
        for (i, coord) in coords.iter_mut().enumerate() {
            *coord = self.coords[i] / mag;
        }
        Self { coords }
    }
}
impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut coords = [0.0; N];
        for (i, coord) in coords.iter_mut().enumerate() {
            *coord = self.coords[i] + other.coords[i];
        }
        Self { coords }
    }
}
impl<const N: usize> Sub for Vector<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut coords = [0.0; N];
        for (i, coord) in coords.iter_mut().enumerate() {
            *coord = self.coords[i] - other.coords[i];
        }
        Self { coords }
    }
}
impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut coords = self.coords;
        for coord in coords.iter_mut() {
            *coord *= rhs;
        }
        Self { coords }
    }
}
impl<const N: usize> Mul<Vector<N>> for f64 {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        let mut coords = rhs.coords;
        for coord in coords.iter_mut() {
            *coord *= self;
        }
        Vector::<N> { coords }
    }
}
