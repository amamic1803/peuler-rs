//! This module provides functions to generate mathematical sequences.

use num_traits::{ConstOne, ConstZero, NumCast, PrimInt};

/// Trait for sequences that can be iterated over.
pub trait Sequence<T>: Iterator {
    /// Sums the next `n` elements of the sequence.
    /// Advances the iterator by `n` elements.
    /// # Arguments
    /// * `n` - The number of elements to sum.
    /// # Returns
    /// * The sum of the next `n` elements in the sequence.
    fn sum_next_n(&mut self, n: usize) -> T;
}

/// The Collatz sequence starting at a given number.
/// The sequence starts with the given number and ends with 1.
/// # Example
/// ```
/// use peuler::math::sequence::CollatzSeq;
///
/// assert_eq!(CollatzSeq::new(13).collect::<Vec<_>>(), vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
/// ```
pub struct CollatzSeq<T> {
    current: T,
    t2: T,
    t3: T,
}
impl<T> CollatzSeq<T>
where
    T: NumCast,
{
    /// Creates a new Collatz sequence starting at the given number.
    /// # Arguments
    /// * `n` - The number to start the Collatz sequence at.
    /// # Returns
    /// * The iterator over the Collatz sequence.
    pub fn new(n: T) -> Self {
        Self {
            current: n,
            t2: T::from(2).unwrap(),
            t3: T::from(3).unwrap(),
        }
    }
}
impl<T> Iterator for CollatzSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == T::ZERO {
            return None;
        }
        let value = self.current;
        if self.current % self.t2 == T::ZERO {
            self.current = self.current / self.t2;
        } else if self.current == T::ONE {
            self.current = T::ZERO; // ends the sequence
        } else {
            self.current = self.t3 * self.current + T::ONE;
        }
        Some(value)
    }
}
impl<T> Sequence<T> for CollatzSeq<T>
where T: PrimInt + ConstZero + ConstOne
{
    fn sum_next_n(&mut self, n: usize) -> T {
        self.take(n).fold(T::ZERO, |acc, x| acc + x)
    }
}

/// The Fibonacci sequence.
/// # Example
/// ```
/// use peuler::math::sequence::FibonacciSeq;
///
/// assert_eq!(FibonacciSeq::new().take(10).collect::<Vec<u64>>(), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
/// assert_eq!(FibonacciSeq::from_index(10).take(10).collect::<Vec<u64>>(), vec![55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181]);
/// ```
pub struct FibonacciSeq<T> {
    curr: T,
    next: T,
    index: usize,
}
impl<T> Default for FibonacciSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> FibonacciSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    /// Creates a new Fibonacci sequence starting from 0.
    /// # Returns
    /// * A new Fibonacci sequence iterator.
    pub fn new() -> Self {
        Self {
            curr: T::ZERO,
            next: T::ONE,
            index: 0,
        }
    }

    /// Creates a Fibonacci sequence starting from the nth number.
    /// # Arguments
    /// * `n` - The index of the Fibonacci number to start from.
    /// # Returns
    /// * A Fibonacci sequence iterator starting from the nth number.
    /// # Example
    /// ```
    /// use peuler::math::sequence::FibonacciSeq;
    ///
    /// // 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
    /// // 21 is at index 8, so:
    /// assert_eq!(FibonacciSeq::<u64>::from_index(8).next().unwrap(), 21);
    /// ```
    pub fn from_index(n: usize) -> Self {
        let mut seq = Self::new();
        if n == 0 {
            return seq;
        }
        seq.nth(n - 1);
        seq
    }
}
impl<T> Iterator for FibonacciSeq<T>
where
    T: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.curr;
        [self.curr, self.next] = [self.next, self.curr + self.next];
        self.index += 1;
        Some(value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        // Binet's formula for Fibonacci numbers
        self.index = n;
        let n = n as f64;
        let sqrt5 = 5f64.sqrt();
        let a = (1.0 + sqrt5) / 2.0;
        let b = (1.0 - sqrt5) / 2.0;
        self.curr = T::from(((a.powf(n) - b.powf(n)) / sqrt5).round()).expect("Overflow in Fibonacci calculation");
        self.next = T::from(((a.powf(n + 1.0) - b.powf(n + 1.0)) / sqrt5).round()).expect("Overflow in Fibonacci calculation");
        self.next()
    }
}
impl<T> Sequence<T> for FibonacciSeq<T> where T: PrimInt + ConstZero {
    fn sum_next_n(&mut self, n: usize) -> T {
        self.take(n).fold(T::ZERO, |acc, x| acc + x)
    }
}

/// The sequence of natural numbers starting from 1.
/// # Example
/// ```
/// use peuler::math::sequence::{NaturalNumbersSeq, Sequence};
///
/// let mut nat = NaturalNumbersSeq::<u32>::new();
/// assert_eq!(nat.take(10).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// assert_eq!(nat.sum_next_n(10), 155);
/// ```
pub struct NaturalNumbersSeq<T> {
    current: T
}
impl<T> Default for NaturalNumbersSeq<T>
where
    T: ConstOne,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> NaturalNumbersSeq<T>
where
    T: ConstOne,
{
    pub fn new() -> Self {
        Self { current: T::ONE }
    }
}
impl<T> Iterator for NaturalNumbersSeq<T>
where
    T: PrimInt + ConstOne,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current;
        self.current = self.current + T::ONE;
        Some(value)
    }
}
impl<T> Sequence<T> for NaturalNumbersSeq<T>
where
    T: PrimInt + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        let t2 = T::from(2).unwrap();
        let curr_sum = self.current * (self.current - T::ONE) / t2;
        let tn = T::from(n).unwrap();
        let next_sum = tn * (tn + T::ONE) / t2;
        let _ = self.take(n);
        next_sum - curr_sum
    }
}

/// The sequence of natural numbers starting from 0.
/// # Example
/// ```
/// use peuler::math::sequence::{NaturalNumbersWithZeroSeq, Sequence};
///
/// let mut nat_zero = NaturalNumbersWithZeroSeq::<u32>::new();
/// assert_eq!(nat_zero.sum_next_n(5), 10);
/// assert_eq!(nat_zero.take(10).collect::<Vec<_>>(), vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
/// ```
pub struct NaturalNumbersWithZeroSeq<T> {
    current: T,
}
impl<T> Default for NaturalNumbersWithZeroSeq<T>
where
    T: ConstZero,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> NaturalNumbersWithZeroSeq<T>
where
    T: ConstZero,
{
    pub fn new() -> Self {
        Self { current: T::ZERO }
    }
}
impl<T> Iterator for NaturalNumbersWithZeroSeq<T>
where
    T: PrimInt + ConstOne,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current;
        self.current = self.current + T::ONE;
        Some(value)
    }
}
impl<T> Sequence<T> for NaturalNumbersWithZeroSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        let tn = T::from(n).unwrap();
        let t2 = T::from(2).unwrap();
        if self.current == T::ZERO {
            let _ = self.take(n);
            tn * (tn - T::ONE) / t2
        } else {
            let curr_sum = self.current * (self.current - T::ONE) / t2;
            let next_sum = tn * (tn + T::ONE) / t2;
            let _ = self.take(n);
            next_sum - curr_sum
        }
    }
}

/// The sequence of squares of natural numbers starting from 1.
pub struct NaturalNumbersSquaredSeq<T> {
    current: T,
}
impl<T> Default for NaturalNumbersSquaredSeq<T>
where
    T: ConstOne,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> NaturalNumbersSquaredSeq<T>
where
    T: ConstOne,
{
    pub fn new() -> Self {
        Self { current: T::ONE }
    }
}
impl<T> Iterator for NaturalNumbersSquaredSeq<T>
where
    T: PrimInt + ConstOne,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current * self.current;
        self.current = self.current + T::ONE;
        Some(value)
    }
}
impl<T> Sequence<T> for NaturalNumbersSquaredSeq<T>
where
    T: PrimInt + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        let tn = T::from(n).unwrap();
        let t2 = T::from(2).unwrap();
        let t6 = T::from(6).unwrap();
        let curr_sum = (self.current - T::ONE) * self.current * (t2 * self.current - T::ONE) / t6;
        let next_sum = tn * (tn + T::ONE) * (tn * t2 + T::ONE) / t6;
        let _ = self.take(n);
        next_sum - curr_sum
    }
}

pub struct NaturalNumbersWithZeroSquaredSeq<T> {
    current: T,
}
impl<T> Default for NaturalNumbersWithZeroSquaredSeq<T>
where
    T: ConstZero,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> NaturalNumbersWithZeroSquaredSeq<T>
where
    T: ConstZero,
{
    pub fn new() -> Self {
        Self { current: T::ZERO }
    }
}
impl<T> Iterator for NaturalNumbersWithZeroSquaredSeq<T>
where
    T: PrimInt + ConstOne,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current * self.current;
        self.current = self.current + T::ONE;
        Some(value)
    }
}
impl<T> Sequence<T> for NaturalNumbersWithZeroSquaredSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        let tn = T::from(n).unwrap();
        let t2 = T::from(2).unwrap();
        let t6 = T::from(6).unwrap();
        if self.current == T::ZERO {
            let _ = self.take(n);
            tn * (tn - T::ONE) * (tn * t2 - T::ONE) / t6
        } else {
            let curr_sum = self.current * (self.current - T::ONE) * (t2 * self.current - T::ONE) / t6;
            let next_sum = tn * (tn + T::ONE) * (tn * t2 + T::ONE) / t6;
            let _ = self.take(n);
            next_sum - curr_sum
        }
    }
}






pub struct EvenNaturalNumbersSeq<T> {
    current: T,
}

pub struct OddNaturalNumbersSeq<T> {
    current: T,
}



pub struct EvenNaturalNumbersWithZeroSeq<T> {
    current: T,
}