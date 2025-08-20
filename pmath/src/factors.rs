//! Factors and divisors of integers.

use crate::primes::sieve_of_eratosthenes;
use num_traits::{ConstOne, ConstZero, PrimInt};
use std::iter::Sum;
use std::iter::from_fn;
use std::vec::IntoIter;

/// An iterator over prime factors of an integer.
///
/// Prime factors are yielded in ascending order.
/// # Example
/// ```
/// use pmath::factors::PrimeFactors;
///
/// let mut iter = PrimeFactors::new(12);
/// assert_eq!(iter.collect::<Vec<_>>(), vec![2, 2, 3]);
///
/// iter = PrimeFactors::new(28);
/// assert_eq!(iter.collect::<Vec<_>>(), vec![2, 2, 7]);
///
/// iter = PrimeFactors::new(2);
/// assert_eq!(iter.collect::<Vec<_>>(), vec![2]);
///
/// iter = PrimeFactors::new(500);
/// assert_eq!(iter.collect::<Vec<_>>(), vec![2, 2, 5, 5, 5]);
/// ```
#[derive(Clone)]
pub struct PrimeFactors<T> {
    n: T,
    factor: T,
    prime_table: IntoIter<T>,
}
impl<T: PrimInt + ConstZero + ConstOne> PrimeFactors<T> {
    /// Create a new [PrimeFactors] iterator for the given integer.
    /// # Arguments
    /// * `n` - The integer to find the prime factors of.
    /// # Returns
    /// * An iterator over the prime factors of the integer in ascending order.
    /// # Panics
    /// * If `n` is negative.
    /// * If `n` cannot be converted to [f64].
    pub fn new(n: T) -> Self {
        if n < T::ZERO {
            panic!("Cannot find prime factors of negative numbers.");
        }
        // calculate primes that are less than or equal to the square root of n
        let mut prime_table = sieve_of_eratosthenes(
            T::from(n.to_f64().expect("Cannot convert to f64").sqrt().floor()).unwrap(),
        )
        .into_iter();
        let factor = prime_table.next().unwrap_or(T::from(2).unwrap());
        Self {
            n,
            factor,
            prime_table,
        }
    }
}
impl<T: PrimInt + ConstZero + ConstOne> Iterator for PrimeFactors<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.n >= self.factor {
            if self.n % self.factor == T::ZERO {
                self.n = self.n / self.factor;
                return Some(self.factor);
            }
            match self.prime_table.next() {
                Some(next_factor) => {
                    self.factor = next_factor;
                }
                // if there are no more primes, n is either a 1 or a prime number (the last factor)
                None => {
                    if self.n != T::ONE {
                        self.factor = self.n;
                    } else {
                        return None;
                    }
                }
            }
        }
        None
    }
}

/// An iterator over distinct prime factors of an integer.
///
/// Distinct prime factors are yielded in ascending order.
/// With each factor, its multiplicity is also yielded.
/// # Example
/// ```
/// use pmath::factors::DistinctPrimeFactors;
///
/// let mut iter = DistinctPrimeFactors::new(12);
/// assert_eq!(iter.collect::<Vec<_>>(), vec![(2, 2), (3, 1)]);
///
/// iter = DistinctPrimeFactors::new(28);
/// assert_eq!(iter.collect::<Vec<_>>(), vec![(2, 2), (7, 1)]);
///
/// iter = DistinctPrimeFactors::new(2);
/// assert_eq!(iter.collect::<Vec<_>>(), vec![(2, 1)]);
///
/// iter = DistinctPrimeFactors::new(500);
/// assert_eq!(iter.collect::<Vec<_>>(), vec![(2, 2), (5, 3)]);
/// ```
#[derive(Clone)]
pub struct DistinctPrimeFactors<T> {
    prime_factors: PrimeFactors<T>,
    factor: T,
    factor_count: usize,
}
impl<T: PrimInt + ConstZero + ConstOne> DistinctPrimeFactors<T> {
    /// Create a new [DistinctPrimeFactors] iterator for the given integer.
    /// # Arguments
    /// * `n` - The integer to find the distinct prime factors of.
    /// # Returns
    /// * An iterator over the distinct prime factors of the integer and
    ///   their multiplicities in ascending order.
    /// # Panics
    /// * If `n` is negative.
    /// * If `n` cannot be converted to [f64].
    pub fn new(n: T) -> Self {
        let prime_factors = PrimeFactors::new(n);
        let factor = T::ZERO;
        let factor_count = 0;
        Self {
            prime_factors,
            factor,
            factor_count,
        }
    }
}
impl<T: PrimInt + ConstZero + ConstOne> Iterator for DistinctPrimeFactors<T> {
    type Item = (T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.prime_factors.next() {
                Some(next_factor) => {
                    if next_factor == self.factor {
                        self.factor_count += 1;
                    } else {
                        let ret = (self.factor, self.factor_count);
                        self.factor = next_factor;
                        self.factor_count = 1;
                        if ret.1 != 0 {
                            return Some(ret);
                        }
                    }
                }
                None => {
                    return if self.factor_count != 0 {
                        let ret = (self.factor, self.factor_count);
                        self.factor_count = 0;
                        Some(ret)
                    } else {
                        None
                    };
                }
            }
        }
    }
}

/// Create an iterator over the prime factors of an integer.
///
/// This function is a convenience wrapper around [PrimeFactors::new].
/// # Arguments
/// * `n` - The integer to find the prime factors of.
/// # Returns
/// * An iterator over the prime factors of the integer in ascending order.
/// # Panics
/// * If `n` is negative.
/// * If `n` cannot be converted to [f64].
pub fn prime_factors<T: PrimInt + ConstZero + ConstOne>(n: T) -> PrimeFactors<T> {
    PrimeFactors::new(n)
}

/// Create an iterator over the distinct prime factors of an integer.
///
/// This function is a convenience wrapper around [DistinctPrimeFactors::new].
/// # Arguments
/// * `n` - The integer to find the distinct prime factors of.
/// # Returns
/// * An iterator over the distinct prime factors of the integer and
///   their multiplicities in ascending order.
/// # Panics
/// * If `n` is negative.
/// * If `n` cannot be converted to [f64].
pub fn distinct_prime_factors<T: PrimInt + ConstZero + ConstOne>(n: T) -> DistinctPrimeFactors<T> {
    DistinctPrimeFactors::new(n)
}

/// An iterator over the divisors of an integer.
///
/// Divisors are yielded in arbitrary order.
/// # Example
/// ```
/// use pmath::factors::Divisors;
/// use itertools::Itertools;
///
/// let mut iter = Divisors::new(12);
/// assert_eq!(iter.len(), 6);
/// assert_eq!(iter.sorted().collect::<Vec<_>>(), vec![1, 2, 3, 4, 6, 12]);
/// iter = Divisors::new(12);
/// assert_eq!(iter.sum::<i32>(), 28);
///
/// iter = Divisors::new(28);
/// assert_eq!(iter.len(), 6);
/// assert_eq!(iter.sorted().collect::<Vec<_>>(), vec![1, 2, 4, 7, 14, 28]);
/// iter = Divisors::new(28);
/// assert_eq!(iter.sum::<i32>(), 56);
///
/// iter = Divisors::new(2);
/// assert_eq!(iter.len(), 2);
/// assert_eq!(iter.sorted().collect::<Vec<_>>(), vec![1, 2]);
/// iter = Divisors::new(2);
/// assert_eq!(iter.sum::<i32>(), 3);
///
/// iter = Divisors::new(500);
/// assert_eq!(iter.len(), 12);
/// assert_eq!(iter.sorted().collect::<Vec<_>>(), vec![1, 2, 4, 5, 10, 20, 25, 50, 100, 125, 250, 500]);
/// iter = Divisors::new(500);
/// assert_eq!(iter.sum::<i32>(), 1092);
///
/// iter = Divisors::new(1);
/// assert_eq!(iter.len(), 1);
/// assert_eq!(iter.sorted().collect::<Vec<_>>(), vec![1]);
/// iter = Divisors::new(1);
/// assert_eq!(iter.sum::<i32>(), 1);
///
/// iter = Divisors::new(0);
/// assert_eq!(iter.len(), 0);
/// assert!(iter.collect::<Vec<_>>().is_empty());
/// iter = Divisors::new(0);
/// assert_eq!(iter.sum::<i32>(), 0);
/// ```
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Divisors<T> {
    factors: Vec<(T, usize, T, usize)>,
    value: T,
    num_of_divisors: usize,
    curr_num: usize,
    sum_of_divisors: T,
    curr_sum: T,
}
impl<T: PrimInt + ConstZero + ConstOne> Divisors<T> {
    /// Create a new [Divisors] iterator for the given integer.
    /// # Arguments
    /// * `n` - The integer to find the divisors of.
    /// # Returns
    /// * An iterator over the divisors of the integer in arbitrary order.
    /// # Panics
    /// * If `n` is negative.
    /// * If `n` cannot be converted to [f64].
    pub fn new(n: T) -> Self {
        let mut factors = DistinctPrimeFactors::new(n)
            .map(|(prime, count)| (prime, count, T::ONE, 0))
            .collect::<Vec<_>>();
        let value = T::ONE;

        // let n be a natural number
        // we can factorize it as follows:
        // n = p1^a1 * p2^a2 * p3^a3 * ... * pn^an
        // (where p1, p2, ..., pn are prime numbers)
        // each combination of these prime factors gives us a divisor of n
        // where each distinct prime factor can be raised to the power of 0 to ai
        // which is ai + 1 choices for each prime factor
        // if d(n) is the number of divisors of n, it is defined as:
        // d(n) = (a1 + 1) * (a2 + 1) * (a3 + 1) * ... * (an + 1)
        let num_of_divisors = factors.iter().map(|(_, a, _, _)| a + 1).product();

        // by setting index to 1 if n == 0, we ensure that the iterator won't yield 0 as a divisor
        let curr_num = if n == T::ZERO { 1 } else { 0 };

        // let σ(n) be the sum of the divisors of n
        // let p be a prime number
        // then σ(p) = p + 1
        // and σ(p^a) = 1 + p + p^2 + ... + p^a = Σ(k=0, a)p^k = (p^(a + 1) - 1) / (p - 1)
        // we can see that
        // σ(p1^a * p2^b) = 1 + p1 + p1^2 + ... + p1^a + p1*p2 + p1^2*p2 + ... + p1^a*p2 + p1*p2^2 + ... + p1^a*p2^2 + ... + p1^a*p2^b
        // = Σ(k=0, a)p1^k * Σ(k=0, b)p2^k = σ(p1^a) * σ(p2^b)
        // = (p1^(a + 1) - 1) / (p1 - 1) * (p2^(b + 1) - 1) / (p2 - 1)
        // we can also increase this to more than 2 prime factors
        // first we check if n is 0, if it is, then we return 0
        let sum_of_divisors = if n == T::ZERO {
            T::ZERO
        } else {
            // if n is not zero, then we proceed to find the sum of the divisors
            // note that σ(1) = 1
            // for each prime factor we calculate the sum of the divisors of that factor
            // and multiply them together

            let mut current_sum = T::ONE;
            for fact in &factors {
                let p = fact.0;
                let a = fact.1;
                current_sum = current_sum * (p.pow(a as u32 + 1) - T::ONE) / (p - T::ONE);
            }
            current_sum
        };

        let curr_sum = T::ZERO;

        // if n == 1, add artificial prime factor 1 so that the .next() method works correctly
        if n == T::ONE {
            factors.push((T::ONE, 1, T::ONE, 0));
        }

        Self {
            factors,
            value,
            num_of_divisors,
            curr_num,
            sum_of_divisors,
            curr_sum,
        }
    }
}
impl<T: PrimInt + ConstOne + Sum<T>> Iterator for Divisors<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_num == self.num_of_divisors {
            return None;
        }

        let ret_value = self.value;
        self.curr_num += 1;
        self.curr_sum = self.curr_sum + ret_value;

        if self.curr_num < self.num_of_divisors {
            let mut i = self.factors.len() - 1;
            loop {
                if self.factors[i].3 < self.factors[i].1 {
                    self.factors[i].3 += 1;
                    self.factors[i].2 = self.factors[i].2 * self.factors[i].0;
                    self.value = self.value * self.factors[i].0;
                    break;
                } else {
                    self.factors[i].3 = 0;
                    self.value = self.value / self.factors[i].2;
                    self.factors[i].2 = T::ONE;
                    i -= 1;
                }
            }
        }

        Some(ret_value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.num_of_divisors - self.curr_num;
        (size, Some(size))
    }

    fn count(self) -> usize {
        self.len()
    }

    fn sum<S: Sum<Self::Item>>(self) -> S {
        let mut returned = false;
        Sum::sum(from_fn(|| {
            if returned {
                None
            } else {
                returned = true;
                Some(self.sum_of_divisors - self.curr_sum)
            }
        }))
    }
}
impl<T: PrimInt + ConstOne + Sum<T>> ExactSizeIterator for Divisors<T> {}

/// An iterator over the proper divisors of an integer.
///
/// Proper divisors are all divisors of an integer except the integer itself.
/// They are yielded in arbitrary order.
/// # Example
/// ```
/// use pmath::factors::ProperDivisors;
/// use itertools::Itertools;
///
/// let mut iter = ProperDivisors::new(12);
/// assert_eq!(iter.len(), 5);
/// assert_eq!(iter.sorted().collect::<Vec<_>>(), vec![1, 2, 3, 4, 6]);
/// iter = ProperDivisors::new(12);
/// assert_eq!(iter.sum::<i32>(), 16);
///
/// iter = ProperDivisors::new(28);
/// assert_eq!(iter.len(), 5);
/// assert_eq!(iter.sorted().collect::<Vec<_>>(), vec![1, 2, 4, 7, 14]);
/// iter = ProperDivisors::new(28);
/// assert_eq!(iter.sum::<i32>(), 28);
///
/// iter = ProperDivisors::new(2);
/// assert_eq!(iter.len(), 1);
/// assert_eq!(iter.sorted().collect::<Vec<_>>(), vec![1]);
/// iter = ProperDivisors::new(2);
/// assert_eq!(iter.sum::<i32>(), 1);
///
/// iter = ProperDivisors::new(500);
/// assert_eq!(iter.len(), 11);
/// assert_eq!(iter.sorted().collect::<Vec<_>>(), vec![1, 2, 4, 5, 10, 20, 25, 50, 100, 125, 250]);
/// iter = ProperDivisors::new(500);
/// assert_eq!(iter.sum::<i32>(), 592);
///
/// iter = ProperDivisors::new(1);
/// assert_eq!(iter.len(), 0);
/// assert!(iter.sorted().collect::<Vec<_>>().is_empty());
/// iter = ProperDivisors::new(1);
/// assert_eq!(iter.sum::<i32>(), 0);
///
/// iter = ProperDivisors::new(0);
/// assert_eq!(iter.len(), 0);
/// assert!(iter.collect::<Vec<_>>().is_empty());
/// iter = ProperDivisors::new(0);
/// assert_eq!(iter.sum::<i32>(), 0);
/// ```
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ProperDivisors<T> {
    n: T,
    divisors: Divisors<T>,
}
impl<T: PrimInt + ConstZero + ConstOne> ProperDivisors<T> {
    /// Create a new [ProperDivisors] iterator for the given integer.
    /// # Arguments
    /// * `n` - The integer to find the proper divisors of.
    /// # Returns
    /// * An iterator over the proper divisors of the integer in arbitrary order.
    /// # Panics
    /// * If `n` is negative.
    /// * If `n` cannot be converted to [f64].
    pub fn new(n: T) -> Self {
        let divisors = Divisors::new(n);
        Self { n, divisors }
    }
}
impl<T: PrimInt + ConstZero + ConstOne + Sum<T>> Iterator for ProperDivisors<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.divisors.next() {
            Some(val) => {
                if val != self.n {
                    Some(val)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.divisors.len().saturating_sub(1);
        (size, Some(size))
    }

    fn count(self) -> usize {
        self.len()
    }

    fn sum<S: Sum<Self::Item>>(self) -> S {
        let mut divisors_sum = self.divisors.sum();
        if divisors_sum > T::ZERO {
            divisors_sum = divisors_sum - self.n;
        }
        let mut returned = false;
        Sum::sum(from_fn(|| {
            if returned {
                None
            } else {
                returned = true;
                Some(divisors_sum)
            }
        }))
    }
}
impl<T: PrimInt + ConstZero + ConstOne + Sum<T>> ExactSizeIterator for ProperDivisors<T> {}

/// Create an iterator over the divisors of an integer.
///
/// This function is a convenience wrapper around [Divisors::new].
/// # Arguments
/// * `n` - The integer to find the divisors of.
/// # Returns
/// * An iterator over the divisors of the integer in arbitrary order.
/// # Panics
/// * If `n` is negative.
/// * If `n` cannot be converted to [f64].
pub fn divisors<T: PrimInt + ConstZero + ConstOne>(n: T) -> Divisors<T> {
    Divisors::new(n)
}

/// Create an iterator over the proper divisors of an integer.
///
/// Proper divisors are all divisors of an integer except the integer itself.
/// This function is a convenience wrapper around [ProperDivisors::new].
/// # Arguments
/// * `n` - The integer to find the proper divisors of.
/// # Returns
/// * An iterator over the proper divisors of the integer in arbitrary order.
/// # Panics
/// * If `n` is negative.
/// * If `n` cannot be converted to [f64].
pub fn proper_divisors<T: PrimInt + ConstZero + ConstOne>(n: T) -> ProperDivisors<T> {
    ProperDivisors::new(n)
}

/// The number of divisors of integers from `0` to `n`.
/// # Arguments
/// * `n` - The integer up to which to calculate the number of divisors.
/// # Returns
/// * The number of divisors of integers from `0` to `n`.
///   Index represents the integer, and the value at that index represents the number of divisors.
/// # Panics
/// * If `n` is negative.
/// * If `n` cannot be converted to [usize].
/// # Example
/// ```
/// use pmath::factors::num_of_divisors_0_to_n;
///
/// assert_eq!(num_of_divisors_0_to_n(10), vec![0, 1, 2, 2, 3, 2, 4, 2, 4, 3, 4]);
/// ```
pub fn num_of_divisors_0_to_n<T>(n: T) -> Vec<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    if n < T::ZERO {
        panic!("Cannot find divisors of negative numbers.");
    }
    let n = n.to_usize().expect("Cannot convert n to usize.");
    let mut divisors = vec![T::ONE; n + 1];
    divisors[0] = T::ZERO;
    for i in 2..=n {
        for j in (i..=n).step_by(i) {
            divisors[j] = divisors[j] + T::ONE;
        }
    }
    divisors
}

/// The number of proper divisors of integers from `0` to `n`.
///
/// Proper divisors are all divisors of an integer except the integer itself.
/// # Arguments
/// * `n` - The integer up to which to calculate the number of proper divisors.
/// # Returns
/// * The number of proper divisors of integers from `0` to `n`.
///   Index represents the integer,
///   and the value at that index represents the number of proper divisors.
/// # Panics
/// * If `n` is negative.
/// * If `n` cannot be converted to [usize].
/// # Example
/// ```
/// use pmath::factors::num_of_proper_divisors_0_to_n;
///
/// assert_eq!(num_of_proper_divisors_0_to_n(10), vec![0, 0, 1, 1, 2, 1, 3, 1, 3, 2, 3]);
/// ```
pub fn num_of_proper_divisors_0_to_n<T>(n: T) -> Vec<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    if n < T::ZERO {
        panic!("Cannot find proper divisors of negative numbers.");
    }
    let n = n.to_usize().expect("Cannot convert n to usize.");
    let mut divisors = vec![T::ZERO; n + 1];
    for i in 2..=n {
        for j in (i..=n).step_by(i) {
            divisors[j] = divisors[j] + T::ONE;
        }
    }
    divisors
}

/// The sum of divisors of integers from `0` to `n`.
/// # Arguments
/// * `n` - The integer up to which to calculate the sum of divisors.
/// # Returns
/// * The sum of the divisors of integers from `0` to `n`.
///   Index represents the integer, and the value at that index represents the sum of divisors.
/// # Panics
/// * If `n` is negative.
/// * If `n` cannot be converted to [usize].
/// # Example
/// ```
/// use pmath::factors::sum_of_divisors_0_to_n;
///
/// assert_eq!(sum_of_divisors_0_to_n(1), vec![0, 1]);
/// assert_eq!(sum_of_divisors_0_to_n(2), vec![0, 1, 3]);
/// assert_eq!(sum_of_divisors_0_to_n(10), vec![0, 1, 3, 4, 7, 6, 12, 8, 15, 13, 18]);
/// ```
pub fn sum_of_divisors_0_to_n<T>(n: T) -> Vec<T>
where
    T: PrimInt + ConstZero,
{
    if n < T::ZERO {
        panic!("Cannot find divisors of negative numbers.");
    }
    let n = n.to_usize().expect("Cannot convert n to usize.");
    let mut divisors = vec![T::ZERO; n + 1];
    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            divisors[j] = divisors[j] + T::from(i).unwrap();
        }
    }
    divisors
}

/// The sum of proper divisors of integers from `0` to `n`.
///
/// Proper divisors are all divisors of an integer except the integer itself.
/// # Arguments
/// * `n` - The integer up to which to calculate the sum of proper divisors.
/// # Returns
/// * The sum of the proper divisors of integers from `0` to `n`.
///   Index represents the integer,
///   and the value at that index represents the sum of proper divisors.
/// # Panics
/// * If `n` is negative.
/// * If `n` cannot be converted to [usize].
/// # Example
/// ```
/// use pmath::factors::sum_of_proper_divisors_0_to_n;
///
/// assert_eq!(sum_of_proper_divisors_0_to_n(1), vec![0, 0]);
/// assert_eq!(sum_of_proper_divisors_0_to_n(2), vec![0, 0, 1]);
/// assert_eq!(sum_of_proper_divisors_0_to_n(10), vec![0, 0, 1, 1, 3, 1, 6, 1, 7, 4, 8]);
/// ```
pub fn sum_of_proper_divisors_0_to_n<T>(n: T) -> Vec<T>
where
    T: PrimInt + ConstZero,
{
    if n < T::ZERO {
        panic!("Cannot find proper divisors of negative numbers.");
    }
    let n = n.to_usize().expect("Cannot convert n to usize.");
    let mut divisors = vec![T::ZERO; n + 1];
    for i in 1..=n {
        for j in ((2 * i)..=n).step_by(i) {
            divisors[j] = divisors[j] + T::from(i).unwrap();
        }
    }
    divisors
}
