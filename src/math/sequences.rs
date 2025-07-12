//! Iterators for various mathematical sequences.

use num_traits::{ConstOne, ConstZero, NumCast, PrimInt};

/// A trait for mathematical sequences that can be iterated over.
pub trait Sequence<T>: Iterator {
    /// Sums the next `n` elements of the sequence.
    /// Advances the iterator by `n` elements.
    /// If the iterator has fewer than `n` elements left, it sums as many as possible.
    /// This should be used instead of `.take(n).sum()` because
    /// some sequences may have a more efficient way to compute the sum.
    /// # Arguments
    /// * `n` - The number of elements to sum.
    /// # Returns
    /// * The sum of the next `n` elements in the sequence.
    fn sum_next_n(&mut self, n: usize) -> T;
}

#[cfg_attr(doc, katexit::katexit)]
/// The Collatz sequence.
///
/// Defined as:
/// $$
///     \\begin{align*}
///         &a\_0 = x \\\\
///         &a\_n =
///         \\begin{cases}
///             \\frac{a\_{n-1}}{2} & \\text{if}\\quad a\_{n-1} \\equiv 0 \\pmod{2} \\\\
///             3 a\_{n-1} + 1 & \\text{if}\\quad a\_{n-1} \\equiv 1 \\pmod{2} \\\\
///             \\nexists & \\text{if}\\quad a\_{n-1} = 1
///         \\end{cases}
///     \\end{align*}
/// $$
/// # Example
/// ```
/// use peuler::math::sequences::CollatzSeq;
///
/// assert_eq!(CollatzSeq::new(13).collect::<Vec<_>>(), vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CollatzSeq<T> {
    current: T,
    t2: T,
    t3: T,
}
impl<T> CollatzSeq<T>
where
    T: NumCast,
{
    /// Creates a new Collatz sequence starting at the given number `n`.
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
        let mut sum = T::ZERO;
        for _ in 0..n {
            match self.next() {
                Some(x) => {
                    sum = sum + x;
                },
                None => {
                    break;
                }
            }
        }
        sum
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// The Fibonacci sequence.
///
/// Defined as:
/// $$
///     \\begin{align*}
///         &a\_0 = 0 \\\\
///         &a\_1 = 1 \\\\
///         &a\_n = a\_{n-1} + a\_{n-2} & \\text{for}\\quad n > 1
///     \\end{align*}
/// $$
/// # Example
/// ```
/// use peuler::math::sequences::FibonacciSeq;
///
/// assert_eq!(FibonacciSeq::new().take(10).collect::<Vec<u64>>(), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
    /// Creates a new Fibonacci sequence starting from `0`.
    /// # Returns
    /// * A new Fibonacci sequence iterator.
    pub fn new() -> Self {
        Self {
            curr: T::ZERO,
            next: T::ONE,
            index: 0,
        }
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
        self.index += n;
        let n = self.index as f64;
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
        let mut sum = T::ZERO;
        for _ in 0..n {
            match self.next() {
                Some(x) => {
                    sum = sum + x;
                },
                None => {
                    break;
                }
            }
        }
        sum
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// The natural numbers sequence.
///
/// Defined as:
/// $$
///     \\begin{align*}
///         &a\_0 = 1 \\\\
///         &a\_n = a\_{n-1} + 1 & \\text{for}\\quad n > 0
///     \\end{align*}
/// $$
/// # Example
/// ```
/// use peuler::math::sequences::{NaturalNumbersSeq, Sequence};
///
/// let mut nat = NaturalNumbersSeq::<u32>::new();
/// let mut seq = Vec::new();
/// for _ in 0..10 {
///   seq.push(nat.next().unwrap());
/// }
/// assert_eq!(seq, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// assert_eq!(nat.sum_next_n(10), 155);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
    /// Creates a new natural numbers sequence starting from `1`.
    /// # Returns
    /// * A new natural numbers sequence iterator.
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

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = self.current + T::from(n).expect("Overflow in nth calculation in NaturalNumbersSeq");
        self.next()
    }
}
impl<T> Sequence<T> for NaturalNumbersSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        if n == 0 {
            return T::ZERO;
        }
        let t2 = T::from(2).unwrap();
        let tn = T::from(n).unwrap() + self.current - T::ONE;
        let curr_sum = self.current * (self.current - T::ONE) / t2;
        let next_sum = tn * (tn + T::ONE) / t2;
        self.nth(n - 1);
        next_sum - curr_sum
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// The natural numbers with zero sequence.
///
/// Defined as:
/// $$
///     \\begin{align*}
///         &a\_0 = 0 \\\\
///         &a\_n = a\_{n-1} + 1 & \\text{for}\\quad n > 0
///     \\end{align*}
/// $$
/// # Example
/// ```
/// use peuler::math::sequences::{NaturalNumbersWithZeroSeq, Sequence};
///
/// let mut nat_zero = NaturalNumbersWithZeroSeq::<u32>::new();
/// assert_eq!(nat_zero.sum_next_n(5), 10);
/// assert_eq!(nat_zero.take(10).collect::<Vec<_>>(), vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
    /// Creates a new natural numbers sequence starting from `0`.
    /// # Returns
    /// * A new natural numbers sequence iterator.
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

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = self.current + T::from(n).expect("Overflow in nth calculation in NaturalNumbersWithZeroSeq");
        self.next()
    }
}
impl<T> Sequence<T> for NaturalNumbersWithZeroSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        if n == 0 {
            return T::ZERO;
        }
        let tn = self.current + T::from(n).unwrap() - T::ONE;
        let t2 = T::from(2).unwrap();
        if self.current == T::ZERO {
            self.nth(n - 1);
            tn * (tn + T::ONE) / t2
        } else {
            let curr_sum = self.current * (self.current - T::ONE) / t2;
            let next_sum = tn * (tn + T::ONE) / t2;
            self.nth(n - 1);
            next_sum - curr_sum
        }
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// The odd natural numbers sequence.
///
/// Defined as:
/// $$
///     \\begin{align*}
///         &a\_0 = 1 \\\\
///         &a\_n = a\_{n-1} + 2 & \\text{for}\\quad n > 0
///     \\end{align*}
/// $$
/// # Example
/// ```
/// use peuler::math::sequences::{OddNaturalNumbersSeq, Sequence};
///
/// let mut odd_nat = OddNaturalNumbersSeq::<i32>::new();
/// assert_eq!(odd_nat.sum_next_n(5), 25);
/// odd_nat.nth(4); // skip 11, 13, 15, 17, 19
/// assert_eq!(odd_nat.take(5).collect::<Vec<_>>(), vec![21, 23, 25, 27, 29]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct OddNaturalNumbersSeq<T> {
    current: T,
}
impl<T> Default for OddNaturalNumbersSeq<T>
where
    T: ConstOne,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> OddNaturalNumbersSeq<T>
where
    T: ConstOne,
{
    /// Creates a new odd natural numbers sequence starting from `1`.
    /// # Returns
    /// * A new odd natural numbers sequence iterator.
    pub fn new() -> Self {
        Self { current: T::ONE }
    }
}
impl<T> Iterator for OddNaturalNumbersSeq<T>
where
    T: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current;
        self.current = self.current + T::from(2).unwrap(); // increment by 2 to get the next odd number
        Some(value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = self.current + T::from(2).unwrap() * T::from(n).expect("Overflow in nth calculation in OddNaturalNumbersSeq");
        self.next()
    }
}
impl<T> Sequence<T> for OddNaturalNumbersSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        if n == 0 {
            return T::ZERO;
        }
        let t2 = T::from(2).unwrap();
        let n_curr = self.current / t2 + T::ONE;
        let n_next = n_curr + T::from(n).unwrap() - T::ONE;
        let curr_sum = (n_curr - T::ONE) * (n_curr - T::ONE);
        let next_sum = n_next * n_next;
        self.nth(n - 1);
        next_sum - curr_sum
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// The even natural numbers sequence.
///
/// Defined as:
/// $$
///     \\begin{align*}
///         &a\_0 = 2 \\\\
///         &a\_n = a\_{n-1} + 2 & \\text{for}\\quad n > 0
///     \\end{align*}
/// $$
/// # Example
/// ```
/// use peuler::math::sequences::{EvenNaturalNumbersSeq, Sequence};
///
/// let mut even_nat = EvenNaturalNumbersSeq::<i32>::new();
/// assert_eq!(even_nat.sum_next_n(5), 30);
/// even_nat.nth(4); // skip 12, 14, 16, 18, 20
/// assert_eq!(even_nat.take(5).collect::<Vec<_>>(), vec![22, 24, 26, 28, 30]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct EvenNaturalNumbersSeq<T> {
    current: T,
}
impl<T> Default for EvenNaturalNumbersSeq<T>
where
    T: NumCast,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> EvenNaturalNumbersSeq<T>
where
    T: NumCast,
{
    /// Creates a new even natural numbers sequence starting from `2`.
    /// # Returns
    /// * A new even natural numbers sequence iterator.
    pub fn new() -> Self {
        Self { current: T::from(2).unwrap() }
    }
}
impl<T> Iterator for EvenNaturalNumbersSeq<T>
where
    T: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current;
        self.current = self.current + T::from(2).unwrap(); // increment by 2 to get the next even number
        Some(value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = self.current + T::from(2).unwrap() * T::from(n).expect("Overflow in nth calculation in EvenNaturalNumbersSeq");
        self.next()
    }
}
impl<T> Sequence<T> for EvenNaturalNumbersSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        if n == 0 {
            return T::ZERO;
        }
        let t2 = T::from(2).unwrap();
        let n_curr = self.current / t2;
        let n_next = n_curr + T::from(n).unwrap() - T::ONE;
        let curr_sum = n_curr * (n_curr - T::ONE);
        let next_sum = n_next * (n_next + T::ONE);
        self.nth(n - 1);
        next_sum - curr_sum
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// The even natural numbers with zero sequence.
///
/// Defined as:
/// $$
///     \\begin{align*}
///         &a\_0 = 0 \\\\
///         &a\_n = a\_{n-1} + 2 & \\text{for}\\quad n > 0
///     \\end{align*}
/// $$
/// # Example
/// ```
/// use peuler::math::sequences::{EvenNaturalNumbersWithZeroSeq, Sequence};
///
/// let mut even_nat = EvenNaturalNumbersWithZeroSeq::<i32>::new();
/// assert_eq!(even_nat.sum_next_n(5), 20);
/// even_nat.nth(5); // skip 10, 12, 14, 16, 18, 20
/// assert_eq!(even_nat.take(5).collect::<Vec<_>>(), vec![22, 24, 26, 28, 30]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct EvenNaturalNumbersWithZeroSeq<T> {
    current: T,
}
impl<T> Default for EvenNaturalNumbersWithZeroSeq<T>
where
    T: ConstZero,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> EvenNaturalNumbersWithZeroSeq<T>
where
    T: ConstZero,
{
    /// Creates a new even natural numbers sequence starting from `0`.
    /// # Returns
    /// * A new even natural numbers sequence iterator.
    pub fn new() -> Self {
        Self { current: T::ZERO }
    }
}
impl<T> Iterator for EvenNaturalNumbersWithZeroSeq<T>
where
    T: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current;
        self.current = self.current + T::from(2).unwrap(); // increment by 2 to get the next even number
        Some(value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = self.current + T::from(2).unwrap() * T::from(n).expect("Overflow in nth calculation in EvenNaturalNumbersWithZeroSeq");
        self.next()
    }
}
impl<T> Sequence<T> for EvenNaturalNumbersWithZeroSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        if n == 0 {
            return T::ZERO;
        }
        let t2 = T::from(2).unwrap();
        let n_curr = self.current / t2;
        let n_next = n_curr + T::from(n).unwrap() - T::ONE;
        if n_curr == T::ZERO {
            self.nth(n - 1);
            n_next * (n_next + T::ONE)
        } else {
            let curr_sum = n_curr * (n_curr - T::ONE);
            let next_sum = n_next * (n_next + T::ONE);
            self.nth(n - 1);
            next_sum - curr_sum
        }
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// The natural numbers squared sequence.
///
/// Defined as:
/// $$
///     a\_n = (n + 1)^2
/// $$
/// # Example
/// ```
/// use peuler::math::sequences::{NaturalNumbersSquaredSeq, Sequence};
///
/// let mut squared_seq = NaturalNumbersSquaredSeq::<u32>::new();
/// assert_eq!(squared_seq.sum_next_n(5), 55);
/// assert_eq!(squared_seq.take(5).collect::<Vec<_>>(), vec![36, 49, 64, 81, 100]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
    /// Creates a new natural numbers squared sequence starting from `1`.
    /// # Returns
    /// * A new natural numbers squared sequence iterator.
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

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = self.current + T::from(n).expect("Overflow in nth calculation in NaturalNumbersSquaredSeq");
        self.next()
    }
}
impl<T> Sequence<T> for NaturalNumbersSquaredSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        if n == 0 {
            return T::ZERO;
        }
        let t2 = T::from(2).unwrap();
        let t6 = T::from(6).unwrap();
        let n_next = self.current + T::from(n).unwrap() - T::ONE;
        let curr_sum = (self.current - T::ONE) * self.current * (t2 * self.current - T::ONE) / t6;
        let next_sum = n_next * (n_next + T::ONE) * (n_next * t2 + T::ONE) / t6;
        self.nth(n - 1);
        next_sum - curr_sum
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// The natural numbers with zero squared sequence.
///
/// Defined as:
/// $$
///     a\_n = n^2
/// $$
/// # Example
/// ```
/// use peuler::math::sequences::{NaturalNumbersWithZeroSquaredSeq, Sequence};
///
/// let mut squared_seq = NaturalNumbersWithZeroSquaredSeq::<u32>::new();
/// assert_eq!(squared_seq.sum_next_n(5), 30);
/// assert_eq!(squared_seq.take(5).collect::<Vec<_>>(), vec![25, 36, 49, 64, 81]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
    /// Creates a new natural numbers squared sequence starting from `0`.
    /// # Returns
    /// * A new natural numbers squared sequence iterator.
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

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = self.current + T::from(n).expect("Overflow in nth calculation in NaturalNumbersWithZeroSquaredSeq");
        self.next()
    }
}
impl<T> Sequence<T> for NaturalNumbersWithZeroSquaredSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        if n == 0 {
            return T::ZERO;
        }
        let t2 = T::from(2).unwrap();
        let t6 = T::from(6).unwrap();
        let n_next = self.current + T::from(n).unwrap() - T::ONE;
        if self.current == T::ZERO {
            self.nth(n - 1);
            n_next * (n_next + T::ONE) * (n_next * t2 + T::ONE) / t6
        } else {
            let curr_sum = self.current * (self.current - T::ONE) * (t2 * self.current - T::ONE) / t6;
            let next_sum = n_next * (n_next + T::ONE) * (n_next * t2 + T::ONE) / t6;
            self.nth(n - 1);
            next_sum - curr_sum
        }
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// The odd natural numbers squared sequence.
///
/// Defined as:
/// $$
///     a\_n = (2n + 1)^2
/// $$
/// # Example
/// ```
/// use peuler::math::sequences::{OddNaturalNumbersSquaredSeq, Sequence};
///
/// let mut squared_seq = OddNaturalNumbersSquaredSeq::<u32>::new();
/// let mut seq = Vec::new();
/// for _ in 0..5 {
///     seq.push(squared_seq.next().unwrap());
/// }
/// assert_eq!(seq, vec![1, 9, 25, 49, 81]);
/// assert_eq!(squared_seq.sum_next_n(5), 1165);  // 121, 169, 225, 289, 361
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct OddNaturalNumbersSquaredSeq<T> {
    current: T,
}
impl<T> Default for OddNaturalNumbersSquaredSeq<T>
where
    T: ConstOne,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> OddNaturalNumbersSquaredSeq<T>
where
    T: ConstOne,
{
    /// Creates a new odd natural numbers squared sequence starting from `1`.
    /// # Returns
    /// * A new odd natural numbers squared sequence iterator.
    pub fn new() -> Self {
        Self { current: T::ONE }
    }
}
impl<T> Iterator for OddNaturalNumbersSquaredSeq<T>
where
    T: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current * self.current; // square the current odd number
        self.current = self.current + T::from(2).unwrap(); // increment by 2 to get the next odd number
        Some(value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = self.current + T::from(2).unwrap() * T::from(n).expect("Overflow in nth calculation in OddNaturalNumbersSquaredSeq");
        self.next()
    }
}
impl<T> Sequence<T> for OddNaturalNumbersSquaredSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        if n == 0 {
            return T::ZERO;
        }
        let t2 = T::from(2).unwrap();
        let t3 = T::from(3).unwrap();
        let n_curr = self.current / t2 + T::ONE;
        let n_next = n_curr + T::from(n).unwrap() - T::ONE;
        let curr_sum = if n_curr > T::ONE {
            (n_curr - T::ONE) * (t2 * n_curr - T::ONE) * (t2 * n_curr - t3) / t3
        } else {
            T::ZERO
        };
        let next_sum = n_next * (t2 * n_next + T::ONE) * (t2 * n_next - T::ONE) / t3;
        self.nth(n - 1);
        next_sum - curr_sum
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// The even natural numbers squared sequence.
///
/// Defined as:
/// $$
///     a\_n = (2n + 2)^2
/// $$
/// # Example
/// ```
/// use peuler::math::sequences::{EvenNaturalNumbersSquaredSeq, Sequence};
///
/// let mut squared_seq = EvenNaturalNumbersSquaredSeq::<u32>::new();
/// let mut seq = Vec::new();
/// for _ in 0..5 {
///     seq.push(squared_seq.next().unwrap());
/// }
/// assert_eq!(seq, vec![4, 16, 36, 64, 100]);
/// assert_eq!(squared_seq.sum_next_n(5), 1320);  // 144, 196, 256, 324, 400
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct EvenNaturalNumbersSquaredSeq<T> {
    current: T,
}
impl<T> Default for EvenNaturalNumbersSquaredSeq<T>
where
    T: NumCast,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> EvenNaturalNumbersSquaredSeq<T>
where
    T: NumCast,
{
    /// Creates a new even natural numbers squared sequence starting from `4`.
    /// # Returns
    /// * A new even natural numbers squared sequence iterator.
    pub fn new() -> Self {
        Self { current: T::from(2).unwrap() }
    }
}
impl<T> Iterator for EvenNaturalNumbersSquaredSeq<T>
where
    T: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current * self.current; // square the current even number
        self.current = self.current + T::from(2).unwrap(); // increment by 2 to get the next even number
        Some(value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = self.current + T::from(2).unwrap() * T::from(n).expect("Overflow in nth calculation in EvenNaturalNumbersSquaredSeq");
        self.next()
    }
}
impl<T> Sequence<T> for EvenNaturalNumbersSquaredSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        if n == 0 {
            return T::ZERO;
        }
        let t2 = T::from(2).unwrap();
        let t3 = T::from(3).unwrap();
        let n_curr = self.current / t2;
        let n_next = n_curr + T::from(n).unwrap() - T::ONE;
        let curr_sum = t2 * (n_curr - T::ONE) * n_curr * (t2 * n_curr - T::ONE) / t3;
        let next_sum = t2 * n_next * (n_next + T::ONE) * (t2 * n_next + T::ONE) / t3;
        self.nth(n - 1);
        next_sum - curr_sum
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// The even natural numbers with zero squared sequence.
///
/// Defined as:
/// $$
///     a\_n = (2n)^2
/// $$
/// # Example
/// ```
/// use peuler::math::sequences::{EvenNaturalNumbersWithZeroSquaredSeq, Sequence};
///
/// let mut squared_seq = EvenNaturalNumbersWithZeroSquaredSeq::<u32>::new();
/// let mut seq = Vec::new();
/// for _ in 0..6 {
///     seq.push(squared_seq.next().unwrap());
/// }
/// assert_eq!(seq, vec![0, 4, 16, 36, 64, 100]);
/// assert_eq!(squared_seq.sum_next_n(5), 1320);  // 144, 196, 256, 324, 400
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct EvenNaturalNumbersWithZeroSquaredSeq<T> {
    current: T,
}
impl<T> Default for EvenNaturalNumbersWithZeroSquaredSeq<T>
where
    T: ConstZero,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> EvenNaturalNumbersWithZeroSquaredSeq<T>
where
    T: ConstZero,
{
    /// Creates a new even natural numbers squared sequence starting from `0`.
    /// # Returns
    /// * A new even natural numbers squared sequence iterator.
    pub fn new() -> Self {
        Self { current: T::ZERO }
    }
}
impl<T> Iterator for EvenNaturalNumbersWithZeroSquaredSeq<T>
where
    T: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current * self.current; // square the current even number
        self.current = self.current + T::from(2).unwrap(); // increment by 2 to get the next even number
        Some(value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = self.current + T::from(2).unwrap() * T::from(n).expect("Overflow in nth calculation in EvenNaturalNumbersWithZeroSquaredSeq");
        self.next()
    }
}
impl<T> Sequence<T> for EvenNaturalNumbersWithZeroSquaredSeq<T>
where
    T: PrimInt + ConstZero + ConstOne,
{
    fn sum_next_n(&mut self, n: usize) -> T {
        if n == 0 {
            return T::ZERO;
        }
        let t2 = T::from(2).unwrap();
        let t3 = T::from(3).unwrap();
        let n_curr = self.current / t2;
        let n_next = n_curr + T::from(n).unwrap() - T::ONE;
        let curr_sum = if n_curr > T::ZERO {
            t2 * (n_curr - T::ONE) * n_curr * (t2 * n_curr - T::ONE) / t3
        } else {
            T::ZERO
        };
        let next_sum = t2 * n_next * (n_next + T::ONE) * (t2 * n_next + T::ONE) / t3;
        self.nth(n - 1);
        next_sum - curr_sum
    }
}
