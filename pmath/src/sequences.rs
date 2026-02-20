//! Iterators over mathematical sequences.

use num_traits::{ConstOne, ConstZero, NumCast, PrimInt};

/// A trait for mathematical sequences that can be iterated over.
pub trait Sequence<T>: Iterator<Item = T>
where
    T: ConstZero,
{
    /// Sum the next `n` elements of the sequence.
    ///
    /// Advances the iterator by `n` elements.
    /// If the iterator has fewer than `n` elements left, it sums as many as possible.
    /// This should be used instead of `.take(n).sum()` because
    /// some sequences have a more efficient implementation for summing elements.
    /// # Arguments
    /// * `n` - The number of elements to sum.
    /// # Returns
    /// * The sum of the next `n` elements in the sequence.
    fn sum_next_n(&mut self, n: usize) -> T {
        let mut sum = T::ZERO;
        for _ in 0..n {
            match self.next() {
                Some(x) => {
                    sum = sum + x;
                }
                None => {
                    break;
                }
            }
        }
        sum
    }
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
/// use pmath::sequences::CollatzSeq;
///
/// assert_eq!(CollatzSeq::new(13).collect::<Vec<_>>(), vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CollatzSeq<T> {
    current: T,
}
impl<T> CollatzSeq<T>
where
    T: PrimInt + ConstOne,
{
    /// Create a new Collatz sequence starting at the integer `n`.
    /// # Arguments
    /// * `n` - The integer to start the Collatz sequence at.
    /// # Returns
    /// * The iterator over the Collatz sequence.
    /// # Panics
    /// * If `n` < `1` since the Collatz sequence requires a positive integer starting point.
    pub fn new(n: T) -> Self {
        if n < T::ONE {
            panic!("Collatz sequence requires a positive integer starting point.");
        }
        Self { current: n }
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
        let t2 = T::from(2).unwrap();
        if self.current % t2 == T::ZERO {
            self.current = self.current / t2;
        } else if self.current == T::ONE {
            self.current = T::ZERO; // ends the sequence
        } else {
            self.current = T::from(3).unwrap() * self.current + T::ONE;
        }
        Some(value)
    }
}
impl<T> Sequence<T> for CollatzSeq<T> where T: PrimInt + ConstZero + ConstOne {}

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
/// use pmath::sequences::FibonacciSeq;
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
    T: ConstZero + ConstOne,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> FibonacciSeq<T>
where
    T: ConstZero + ConstOne,
{
    /// Create a new Fibonacci sequence starting from `0`.
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
        self.curr = T::from(((a.powf(n) - b.powf(n)) / sqrt5).round())
            .expect("Overflow in Fibonacci calculation");
        self.next = T::from(((a.powf(n + 1.0) - b.powf(n + 1.0)) / sqrt5).round())
            .expect("Overflow in Fibonacci calculation");
        self.next()
    }
}
impl<T> Sequence<T> for FibonacciSeq<T> where T: PrimInt + ConstZero {}

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
/// use pmath::sequences::{NatNumSeq, Sequence};
///
/// let mut nat = NatNumSeq::<u32>::new();
/// let mut seq = Vec::new();
/// for _ in 0..10 {
///   seq.push(nat.next().unwrap());
/// }
/// assert_eq!(seq, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// assert_eq!(nat.sum_next_n(10), 155);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct NatNumSeq<T> {
    current: T,
}
impl<T> Default for NatNumSeq<T>
where
    T: ConstOne,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> NatNumSeq<T>
where
    T: ConstOne,
{
    /// Create a natural numbers sequence starting from `1`.
    /// # Returns
    /// * A new natural numbers sequence iterator.
    pub fn new() -> Self {
        Self { current: T::ONE }
    }
}
impl<T> Iterator for NatNumSeq<T>
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
        self.current =
            self.current + T::from(n).expect("Overflow in nth calculation in NaturalNumbersSeq");
        self.next()
    }
}
impl<T> Sequence<T> for NatNumSeq<T>
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
/// use pmath::sequences::{NatNumW0Seq, Sequence};
///
/// let mut nat_zero = NatNumW0Seq::<u32>::new();
/// assert_eq!(nat_zero.sum_next_n(5), 10);
/// assert_eq!(nat_zero.take(10).collect::<Vec<_>>(), vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct NatNumW0Seq<T> {
    current: T,
}
impl<T> Default for NatNumW0Seq<T>
where
    T: ConstZero,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> NatNumW0Seq<T>
where
    T: ConstZero,
{
    /// Create a natural numbers sequence starting from `0`.
    /// # Returns
    /// * A new natural numbers sequence iterator.
    pub fn new() -> Self {
        Self { current: T::ZERO }
    }
}
impl<T> Iterator for NatNumW0Seq<T>
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
        self.current = self.current
            + T::from(n).expect("Overflow in nth calculation in NaturalNumbersWithZeroSeq");
        self.next()
    }
}
impl<T> Sequence<T> for NatNumW0Seq<T>
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
/// use pmath::sequences::{OddNatNumSeq, Sequence};
///
/// let mut odd_nat = OddNatNumSeq::<i32>::new();
/// assert_eq!(odd_nat.sum_next_n(5), 25);
/// odd_nat.nth(4); // skip 11, 13, 15, 17, 19
/// assert_eq!(odd_nat.take(5).collect::<Vec<_>>(), vec![21, 23, 25, 27, 29]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct OddNatNumSeq<T> {
    current: T,
}
impl<T> Default for OddNatNumSeq<T>
where
    T: ConstOne,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> OddNatNumSeq<T>
where
    T: ConstOne,
{
    /// Create an odd natural numbers sequence starting from `1`.
    /// # Returns
    /// * A new odd natural numbers sequence iterator.
    pub fn new() -> Self {
        Self { current: T::ONE }
    }
}
impl<T> Iterator for OddNatNumSeq<T>
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
        self.current = self.current
            + T::from(2).unwrap()
                * T::from(n).expect("Overflow in nth calculation in OddNaturalNumbersSeq");
        self.next()
    }
}
impl<T> Sequence<T> for OddNatNumSeq<T>
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
/// use pmath::sequences::{EvenNatNumSeq, Sequence};
///
/// let mut even_nat = EvenNatNumSeq::<i32>::new();
/// assert_eq!(even_nat.sum_next_n(5), 30);
/// even_nat.nth(4); // skip 12, 14, 16, 18, 20
/// assert_eq!(even_nat.take(5).collect::<Vec<_>>(), vec![22, 24, 26, 28, 30]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct EvenNatNumSeq<T> {
    current: T,
}
impl<T> Default for EvenNatNumSeq<T>
where
    T: NumCast,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> EvenNatNumSeq<T>
where
    T: NumCast,
{
    /// Create an even natural numbers sequence starting from `2`.
    /// # Returns
    /// * A new even natural numbers sequence iterator.
    pub fn new() -> Self {
        Self {
            current: T::from(2).unwrap(),
        }
    }
}
impl<T> Iterator for EvenNatNumSeq<T>
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
        self.current = self.current
            + T::from(2).unwrap()
                * T::from(n).expect("Overflow in nth calculation in EvenNaturalNumbersSeq");
        self.next()
    }
}
impl<T> Sequence<T> for EvenNatNumSeq<T>
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
/// use pmath::sequences::{EvenNatNumW0Seq, Sequence};
///
/// let mut even_nat = EvenNatNumW0Seq::<i32>::new();
/// assert_eq!(even_nat.sum_next_n(5), 20);
/// even_nat.nth(5); // skip 10, 12, 14, 16, 18, 20
/// assert_eq!(even_nat.take(5).collect::<Vec<_>>(), vec![22, 24, 26, 28, 30]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct EvenNatNumW0Seq<T> {
    current: T,
}
impl<T> Default for EvenNatNumW0Seq<T>
where
    T: ConstZero,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> EvenNatNumW0Seq<T>
where
    T: ConstZero,
{
    /// Create an even natural numbers sequence starting from `0`.
    /// # Returns
    /// * A new even natural numbers sequence iterator.
    pub fn new() -> Self {
        Self { current: T::ZERO }
    }
}
impl<T> Iterator for EvenNatNumW0Seq<T>
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
        self.current = self.current
            + T::from(2).unwrap()
                * T::from(n).expect("Overflow in nth calculation in EvenNaturalNumbersWithZeroSeq");
        self.next()
    }
}
impl<T> Sequence<T> for EvenNatNumW0Seq<T>
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
/// use pmath::sequences::{NatNumSqSeq, Sequence};
///
/// let mut squared_seq = NatNumSqSeq::<u32>::new();
/// assert_eq!(squared_seq.sum_next_n(5), 55);
/// assert_eq!(squared_seq.take(5).collect::<Vec<_>>(), vec![36, 49, 64, 81, 100]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct NatNumSqSeq<T> {
    current: T,
}
impl<T> Default for NatNumSqSeq<T>
where
    T: ConstOne,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> NatNumSqSeq<T>
where
    T: ConstOne,
{
    /// Create a natural numbers squared sequence starting from `1`.
    /// # Returns
    /// * A new natural numbers squared sequence iterator.
    pub fn new() -> Self {
        Self { current: T::ONE }
    }
}
impl<T> Iterator for NatNumSqSeq<T>
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
        self.current = self.current
            + T::from(n).expect("Overflow in nth calculation in NaturalNumbersSquaredSeq");
        self.next()
    }
}
impl<T> Sequence<T> for NatNumSqSeq<T>
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
/// use pmath::sequences::{NatNumW0SqSeq, Sequence};
///
/// let mut squared_seq = NatNumW0SqSeq::<u32>::new();
/// assert_eq!(squared_seq.sum_next_n(5), 30);
/// assert_eq!(squared_seq.take(5).collect::<Vec<_>>(), vec![25, 36, 49, 64, 81]);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct NatNumW0SqSeq<T> {
    current: T,
}
impl<T> Default for NatNumW0SqSeq<T>
where
    T: ConstZero,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> NatNumW0SqSeq<T>
where
    T: ConstZero,
{
    /// Create a natural numbers squared sequence starting from `0`.
    /// # Returns
    /// * A new natural numbers squared sequence iterator.
    pub fn new() -> Self {
        Self { current: T::ZERO }
    }
}
impl<T> Iterator for NatNumW0SqSeq<T>
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
        self.current = self.current
            + T::from(n).expect("Overflow in nth calculation in NaturalNumbersWithZeroSquaredSeq");
        self.next()
    }
}
impl<T> Sequence<T> for NatNumW0SqSeq<T>
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
            let curr_sum =
                self.current * (self.current - T::ONE) * (t2 * self.current - T::ONE) / t6;
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
/// use pmath::sequences::{OddNatNumSqSeq, Sequence};
///
/// let mut squared_seq = OddNatNumSqSeq::<u32>::new();
/// let mut seq = Vec::new();
/// for _ in 0..5 {
///     seq.push(squared_seq.next().unwrap());
/// }
/// assert_eq!(seq, vec![1, 9, 25, 49, 81]);
/// assert_eq!(squared_seq.sum_next_n(5), 1165);  // 121, 169, 225, 289, 361
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct OddNatNumSqSeq<T> {
    current: T,
}
impl<T> Default for OddNatNumSqSeq<T>
where
    T: ConstOne,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> OddNatNumSqSeq<T>
where
    T: ConstOne,
{
    /// Create an odd natural numbers squared sequence starting from `1`.
    /// # Returns
    /// * A new odd natural numbers squared sequence iterator.
    pub fn new() -> Self {
        Self { current: T::ONE }
    }
}
impl<T> Iterator for OddNatNumSqSeq<T>
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
        self.current = self.current
            + T::from(2).unwrap()
                * T::from(n).expect("Overflow in nth calculation in OddNaturalNumbersSquaredSeq");
        self.next()
    }
}
impl<T> Sequence<T> for OddNatNumSqSeq<T>
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
/// use pmath::sequences::{EvenNatNumSqSeq, Sequence};
///
/// let mut squared_seq = EvenNatNumSqSeq::<u32>::new();
/// let mut seq = Vec::new();
/// for _ in 0..5 {
///     seq.push(squared_seq.next().unwrap());
/// }
/// assert_eq!(seq, vec![4, 16, 36, 64, 100]);
/// assert_eq!(squared_seq.sum_next_n(5), 1320);  // 144, 196, 256, 324, 400
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct EvenNatNumSqSeq<T> {
    current: T,
}
impl<T> Default for EvenNatNumSqSeq<T>
where
    T: NumCast,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> EvenNatNumSqSeq<T>
where
    T: NumCast,
{
    /// Create an even natural numbers squared sequence starting from `4`.
    /// # Returns
    /// * A new even natural numbers squared sequence iterator.
    pub fn new() -> Self {
        Self {
            current: T::from(2).unwrap(),
        }
    }
}
impl<T> Iterator for EvenNatNumSqSeq<T>
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
        self.current = self.current
            + T::from(2).unwrap()
                * T::from(n).expect("Overflow in nth calculation in EvenNaturalNumbersSquaredSeq");
        self.next()
    }
}
impl<T> Sequence<T> for EvenNatNumSqSeq<T>
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
/// use pmath::sequences::{EvenNatNumW0SqSeq, Sequence};
///
/// let mut squared_seq = EvenNatNumW0SqSeq::<u32>::new();
/// let mut seq = Vec::new();
/// for _ in 0..6 {
///     seq.push(squared_seq.next().unwrap());
/// }
/// assert_eq!(seq, vec![0, 4, 16, 36, 64, 100]);
/// assert_eq!(squared_seq.sum_next_n(5), 1320);  // 144, 196, 256, 324, 400
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct EvenNatNumW0SqSeq<T> {
    current: T,
}
impl<T> Default for EvenNatNumW0SqSeq<T>
where
    T: ConstZero,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> EvenNatNumW0SqSeq<T>
where
    T: ConstZero,
{
    /// Create an even natural numbers squared sequence starting from `0`.
    /// # Returns
    /// * A new even natural numbers squared sequence iterator.
    pub fn new() -> Self {
        Self { current: T::ZERO }
    }
}
impl<T> Iterator for EvenNatNumW0SqSeq<T>
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
        self.current = self.current
            + T::from(2).unwrap()
                * T::from(n)
                    .expect("Overflow in nth calculation in EvenNaturalNumbersWithZeroSquaredSeq");
        self.next()
    }
}
impl<T> Sequence<T> for EvenNatNumW0SqSeq<T>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_sum_next_n() {
        //! Test the default [Sequence::sum_next_n] implementation for sequences that don't override it.

        struct NewSeq {
            current: u32,
        }
        impl NewSeq {
            fn new() -> Self {
                Self { current: 1 }
            }
        }
        impl Iterator for NewSeq {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                let value = self.current;
                self.current += 1;
                Some(value)
            }
        }
        impl Sequence<u32> for NewSeq {}

        let mut seq = NewSeq::new();

        assert_eq!(seq.sum_next_n(0), 0); // no numbers to sum, should return 0
        assert_eq!(seq.sum_next_n(5), 15); // 1 + 2 + 3 + 4 + 5 = 15
        assert_eq!(seq.sum_next_n(25), 450); // 6 + 7 + ... + 30 = 450
    }

    const COLLATZ_SEQ_EXAMPLE: [u32; 9] = [6, 3, 10, 5, 16, 8, 4, 2, 1];

    #[test]
    #[should_panic]
    fn collatz_seq_nonpositive_start() {
        //! Test that creating a [CollatzSeq] with a non-positive starting point panics.

        CollatzSeq::new(0);
    }

    #[test]
    fn collatz_seq_primitive_types() {
        //! Test that the [CollatzSeq] works with different primitive integer types.

        // unsigned types
        assert_eq!(
            CollatzSeq::new(6u8).collect::<Vec<_>>(),
            COLLATZ_SEQ_EXAMPLE.map(|val| val as u8).to_vec()
        );
        assert_eq!(
            CollatzSeq::new(6u16).collect::<Vec<_>>(),
            COLLATZ_SEQ_EXAMPLE.map(|val| val as u16).to_vec()
        );
        assert_eq!(
            CollatzSeq::new(6u32).collect::<Vec<_>>(),
            COLLATZ_SEQ_EXAMPLE.to_vec()
        );
        assert_eq!(
            CollatzSeq::new(6u64).collect::<Vec<_>>(),
            COLLATZ_SEQ_EXAMPLE.map(|val| val as u64).to_vec()
        );
        assert_eq!(
            CollatzSeq::new(6u128).collect::<Vec<_>>(),
            COLLATZ_SEQ_EXAMPLE.map(|val| val as u128).to_vec()
        );
        assert_eq!(
            CollatzSeq::new(6usize).collect::<Vec<_>>(),
            COLLATZ_SEQ_EXAMPLE.map(|val| val as usize).to_vec()
        );

        // signed types
        assert_eq!(
            CollatzSeq::new(6i8).collect::<Vec<_>>(),
            COLLATZ_SEQ_EXAMPLE.map(|val| val as i8).to_vec()
        );
        assert_eq!(
            CollatzSeq::new(6i16).collect::<Vec<_>>(),
            COLLATZ_SEQ_EXAMPLE.map(|val| val as i16).to_vec()
        );
        assert_eq!(
            CollatzSeq::new(6i32).collect::<Vec<_>>(),
            COLLATZ_SEQ_EXAMPLE.map(|val| val as i32).to_vec()
        );
        assert_eq!(
            CollatzSeq::new(6i64).collect::<Vec<_>>(),
            COLLATZ_SEQ_EXAMPLE.map(|val| val as i64).to_vec()
        );
        assert_eq!(
            CollatzSeq::new(6i128).collect::<Vec<_>>(),
            COLLATZ_SEQ_EXAMPLE.map(|val| val as i128).to_vec()
        );
        assert_eq!(
            CollatzSeq::new(6isize).collect::<Vec<_>>(),
            COLLATZ_SEQ_EXAMPLE.map(|val| val as isize).to_vec()
        );
    }

    #[test]
    fn collatz_seq_verify() {
        //! Test that the [CollatzSeq] generates the correct sequence for a given starting point.

        let start = 6;
        let expected_sequence = COLLATZ_SEQ_EXAMPLE.to_vec();
        let generated_sequence: Vec<u32> = CollatzSeq::new(start).collect();
        assert_eq!(generated_sequence, expected_sequence);
    }

    #[test]
    fn collatz_seq_sum_next_n() {
        //! Test the [CollatzSeq::sum_next_n] method for a specific starting point and number of terms.

        let start = 6;
        let mut seq = CollatzSeq::new(start);
        assert_eq!(
            seq.sum_next_n(5),
            COLLATZ_SEQ_EXAMPLE.into_iter().take(5).sum::<u32>()
        );
    }

    const FIBONACCI_SEQ_EXAMPLE: [i32; 40] = [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269,
        2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986,
    ];

    #[test]
    fn fibonacci_seq_primitive_types() {
        //! Test that the [FibonacciSeq] works with different primitive integer types.

        // unsigned types
        assert_eq!(
            FibonacciSeq::<u8>::new().take(10).collect::<Vec<_>>(),
            FIBONACCI_SEQ_EXAMPLE
                .into_iter()
                .take(10)
                .map(|val| val as u8)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            FibonacciSeq::<u16>::new().take(10).collect::<Vec<_>>(),
            FIBONACCI_SEQ_EXAMPLE
                .into_iter()
                .take(10)
                .map(|val| val as u16)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            FibonacciSeq::<u32>::new().take(10).collect::<Vec<_>>(),
            FIBONACCI_SEQ_EXAMPLE
                .into_iter()
                .take(10)
                .map(|val| val as u32)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            FibonacciSeq::<u64>::new().take(10).collect::<Vec<_>>(),
            FIBONACCI_SEQ_EXAMPLE
                .into_iter()
                .take(10)
                .map(|val| val as u64)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            FibonacciSeq::<u128>::new().take(10).collect::<Vec<_>>(),
            FIBONACCI_SEQ_EXAMPLE
                .into_iter()
                .take(10)
                .map(|val| val as u128)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            FibonacciSeq::<usize>::new().take(10).collect::<Vec<_>>(),
            FIBONACCI_SEQ_EXAMPLE
                .into_iter()
                .take(10)
                .map(|val| val as usize)
                .collect::<Vec<_>>()
        );

        // signed types
        assert_eq!(
            FibonacciSeq::<i8>::new().take(10).collect::<Vec<_>>(),
            FIBONACCI_SEQ_EXAMPLE
                .into_iter()
                .take(10)
                .map(|val| val as i8)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            FibonacciSeq::<i16>::new().take(10).collect::<Vec<_>>(),
            FIBONACCI_SEQ_EXAMPLE
                .into_iter()
                .take(10)
                .map(|val| val as i16)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            FibonacciSeq::<i32>::new().take(10).collect::<Vec<_>>(),
            FIBONACCI_SEQ_EXAMPLE
                .into_iter()
                .take(10)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            FibonacciSeq::<i64>::new().take(10).collect::<Vec<_>>(),
            FIBONACCI_SEQ_EXAMPLE
                .into_iter()
                .take(10)
                .map(|val| val as i64)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            FibonacciSeq::<i128>::new().take(10).collect::<Vec<_>>(),
            FIBONACCI_SEQ_EXAMPLE
                .into_iter()
                .take(10)
                .map(|val| val as i128)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            FibonacciSeq::<isize>::new().take(10).collect::<Vec<_>>(),
            FIBONACCI_SEQ_EXAMPLE
                .into_iter()
                .take(10)
                .map(|val| val as isize)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn fibonacci_seq_verify() {
        //! Test that the [FibonacciSeq] generates the correct sequence.

        for n in 0..=FIBONACCI_SEQ_EXAMPLE.len() {
            assert_eq!(
                FibonacciSeq::new().take(n).collect::<Vec<i32>>(),
                FIBONACCI_SEQ_EXAMPLE
                    .iter()
                    .take(n)
                    .cloned()
                    .collect::<Vec<i32>>()
            );
        }
    }

    #[test]
    fn fibonacci_seq_nth() {
        //! Test the [FibonacciSeq::nth] method for various values of `n`.

        for (i, value) in FIBONACCI_SEQ_EXAMPLE.into_iter().enumerate() {
            assert_eq!(FibonacciSeq::<i32>::new().nth(i).unwrap(), value)
        }
    }

    #[test]
    fn fibonacci_seq_sum_next_n() {
        //! Test the [FibonacciSeq::sum_next_n] method for a specific number of terms.

        let mut seq = FibonacciSeq::<i32>::new();
        assert_eq!(seq.sum_next_n(0), 0);
        assert_eq!(seq.sum_next_n(1), 0);
        assert_eq!(
            seq.sum_next_n(FIBONACCI_SEQ_EXAMPLE.len() - 1),
            FIBONACCI_SEQ_EXAMPLE.iter().sum::<i32>()
        );
    }

    #[test]
    fn nat_num_seq_primitive_types() {
        //! Test that the [NatNumSeq] works with different primitive integer types.

        // unsigned types
        assert_eq!(
            NatNumSeq::<u8>::new().take(10).collect::<Vec<_>>(),
            (1u8..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSeq::<u16>::new().take(10).collect::<Vec<_>>(),
            (1u16..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSeq::<u32>::new().take(10).collect::<Vec<_>>(),
            (1u32..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSeq::<u64>::new().take(10).collect::<Vec<_>>(),
            (1u64..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSeq::<u128>::new().take(10).collect::<Vec<_>>(),
            (1u128..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSeq::<usize>::new().take(10).collect::<Vec<_>>(),
            (1usize..=10).collect::<Vec<_>>()
        );

        // signed types
        assert_eq!(
            NatNumSeq::<i8>::new().take(10).collect::<Vec<_>>(),
            (1i8..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSeq::<i16>::new().take(10).collect::<Vec<_>>(),
            (1i16..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSeq::<i32>::new().take(10).collect::<Vec<_>>(),
            (1i32..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSeq::<i64>::new().take(10).collect::<Vec<_>>(),
            (1i64..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSeq::<i128>::new().take(10).collect::<Vec<_>>(),
            (1i128..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSeq::<isize>::new().take(10).collect::<Vec<_>>(),
            (1isize..=10).collect::<Vec<_>>()
        );
    }

    #[test]
    fn nat_num_seq_verify() {
        //! Test that the [NatNumSeq] generates the correct sequence.

        let seq = NatNumSeq::<u32>::new();
        assert_eq!(
            seq.take(10).collect::<Vec<u32>>(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
    }

    #[test]
    fn nat_num_seq_nth() {
        //! Test the [NatNumSeq::nth] method for various values of `n`.

        for n in 0..100 {
            assert_eq!(NatNumSeq::<i32>::new().nth(n).unwrap(), (n + 1) as i32);
        }
    }

    #[test]
    fn nat_num_seq_sum_next_n() {
        //! Test the [NatNumSeq::sum_next_n] method for a specific number of terms.

        let mut seq = NatNumSeq::<u32>::new();
        assert_eq!(seq.sum_next_n(0), 0);
        assert_eq!(seq.sum_next_n(1), 1);
        assert_eq!(seq.sum_next_n(9), 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10);

        let mut sum = 0;
        for n in 1..=100 {
            sum += n;
            assert_eq!(NatNumSeq::<u32>::new().sum_next_n(n), sum as u32);
        }
    }

    #[test]
    fn nat_num_w0_seq_primitive_types() {
        //! Test that the [NatNumW0Seq] works with different primitive integer types.

        // unsigned types
        assert_eq!(
            NatNumW0Seq::<u8>::new().take(11).collect::<Vec<_>>(),
            (0u8..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0Seq::<u16>::new().take(11).collect::<Vec<_>>(),
            (0u16..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0Seq::<u32>::new().take(11).collect::<Vec<_>>(),
            (0u32..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0Seq::<u64>::new().take(11).collect::<Vec<_>>(),
            (0u64..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0Seq::<u128>::new().take(11).collect::<Vec<_>>(),
            (0u128..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0Seq::<usize>::new().take(11).collect::<Vec<_>>(),
            (0usize..=10).collect::<Vec<_>>()
        );

        // signed types
        assert_eq!(
            NatNumW0Seq::<i8>::new().take(11).collect::<Vec<_>>(),
            (0i8..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0Seq::<i16>::new().take(11).collect::<Vec<_>>(),
            (0i16..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0Seq::<i32>::new().take(11).collect::<Vec<_>>(),
            (0i32..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0Seq::<i64>::new().take(11).collect::<Vec<_>>(),
            (0i64..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0Seq::<i128>::new().take(11).collect::<Vec<_>>(),
            (0i128..=10).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0Seq::<isize>::new().take(11).collect::<Vec<_>>(),
            (0isize..=10).collect::<Vec<_>>()
        );
    }

    #[test]
    fn nat_num_w0_seq_verify() {
        //! Test that the [NatNumW0Seq] generates the correct sequence.

        let seq = NatNumW0Seq::<u32>::new();
        assert_eq!(
            seq.take(11).collect::<Vec<u32>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
    }

    #[test]
    fn nat_num_w0_seq_nth() {
        //! Test the [NatNumW0Seq::nth] method for various values of `n`.

        for n in 0..100 {
            assert_eq!(NatNumW0Seq::<i32>::new().nth(n).unwrap(), n as i32);
        }
    }

    #[test]
    fn nat_num_w0_seq_sum_next_n() {
        //! Test the [NatNumW0Seq::sum_next_n] method for a specific number of terms.

        let mut seq = NatNumW0Seq::<u32>::new();
        assert_eq!(seq.sum_next_n(0), 0);
        assert_eq!(seq.sum_next_n(1), 0);
        assert_eq!(seq.sum_next_n(10), 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10);

        let mut sum = 0;
        for n in 1..=100 {
            sum += n;
            assert_eq!(NatNumW0Seq::<u32>::new().sum_next_n(n + 1), sum as u32);
        }
    }

    #[test]
    fn odd_nat_num_seq_primitive_types() {
        //! Test that the [OddNatNumSeq] works with different primitive integer types.

        // unsigned types
        assert_eq!(
            OddNatNumSeq::<u8>::new().take(10).collect::<Vec<_>>(),
            (1u8..=19).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSeq::<u16>::new().take(10).collect::<Vec<_>>(),
            (1u16..=19).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSeq::<u32>::new().take(10).collect::<Vec<_>>(),
            (1u32..=19).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSeq::<u64>::new().take(10).collect::<Vec<_>>(),
            (1u64..=19).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSeq::<u128>::new().take(10).collect::<Vec<_>>(),
            (1u128..=19).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSeq::<usize>::new().take(10).collect::<Vec<_>>(),
            (1usize..=19).step_by(2).collect::<Vec<_>>()
        );

        // signed types
        assert_eq!(
            OddNatNumSeq::<i8>::new().take(10).collect::<Vec<_>>(),
            (1i8..=19).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSeq::<i16>::new().take(10).collect::<Vec<_>>(),
            (1i16..=19).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSeq::<i32>::new().take(10).collect::<Vec<_>>(),
            (1i32..=19).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSeq::<i64>::new().take(10).collect::<Vec<_>>(),
            (1i64..=19).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSeq::<i128>::new().take(10).collect::<Vec<_>>(),
            (1i128..=19).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSeq::<isize>::new().take(10).collect::<Vec<_>>(),
            (1isize..=19).step_by(2).collect::<Vec<_>>()
        );
    }

    #[test]
    fn odd_nat_num_seq_verify() {
        //! Test that the [OddNatNumSeq] generates the correct sequence.

        let seq = OddNatNumSeq::<u32>::new();
        assert_eq!(
            seq.take(10).collect::<Vec<u32>>(),
            vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
        );
    }

    #[test]
    fn odd_nat_num_seq_nth() {
        //! Test the [OddNatNumSeq::nth] method for various values of `n`.

        for n in 0..100 {
            assert_eq!(
                OddNatNumSeq::<i32>::new().nth(n).unwrap(),
                (n * 2 + 1) as i32
            );
        }
    }

    #[test]
    fn odd_nat_num_seq_sum_next_n() {
        //! Test the [OddNatNumSeq::sum_next_n] method for a specific number of terms.

        let mut seq = OddNatNumSeq::<u32>::new();
        assert_eq!(seq.sum_next_n(0), 0);
        assert_eq!(seq.sum_next_n(1), 1);
        assert_eq!(seq.sum_next_n(9), 3 + 5 + 7 + 9 + 11 + 13 + 15 + 17 + 19);

        let mut sum = 0;
        for n in 1..=100 {
            sum += n * 2 - 1;
            assert_eq!(OddNatNumSeq::<u32>::new().sum_next_n(n), sum as u32);
        }
    }

    #[test]
    fn even_nat_num_seq_primitive_types() {
        //! Test that the [EvenNatNumSeq] works with different primitive integer types.

        // unsigned types
        assert_eq!(
            EvenNatNumSeq::<u8>::new().take(10).collect::<Vec<_>>(),
            (2u8..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSeq::<u16>::new().take(10).collect::<Vec<_>>(),
            (2u16..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSeq::<u32>::new().take(10).collect::<Vec<_>>(),
            (2u32..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSeq::<u64>::new().take(10).collect::<Vec<_>>(),
            (2u64..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSeq::<u128>::new().take(10).collect::<Vec<_>>(),
            (2u128..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSeq::<usize>::new().take(10).collect::<Vec<_>>(),
            (2usize..=20).step_by(2).collect::<Vec<_>>()
        );

        // signed types
        assert_eq!(
            EvenNatNumSeq::<i8>::new().take(10).collect::<Vec<_>>(),
            (2i8..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSeq::<i16>::new().take(10).collect::<Vec<_>>(),
            (2i16..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSeq::<i32>::new().take(10).collect::<Vec<_>>(),
            (2i32..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSeq::<i64>::new().take(10).collect::<Vec<_>>(),
            (2i64..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSeq::<i128>::new().take(10).collect::<Vec<_>>(),
            (2i128..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSeq::<isize>::new().take(10).collect::<Vec<_>>(),
            (2isize..=20).step_by(2).collect::<Vec<_>>()
        );
    }

    #[test]
    fn even_nat_num_seq_verify() {
        //! Test that the [EvenNatNumSeq] generates the correct sequence.

        let seq = EvenNatNumSeq::<u32>::new();
        assert_eq!(
            seq.take(10).collect::<Vec<u32>>(),
            vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
        );
    }

    #[test]
    fn even_nat_num_seq_nth() {
        //! Test the [EvenNatNumSeq::nth] method for various values of `n`.

        for n in 0..100 {
            assert_eq!(
                EvenNatNumSeq::<i32>::new().nth(n).unwrap(),
                ((n + 1) * 2) as i32
            );
        }
    }

    #[test]
    fn even_nat_num_seq_sum_next_n() {
        //! Test the [EvenNatNumSeq::sum_next_n] method for a specific number of terms.

        let mut seq = EvenNatNumSeq::<u32>::new();
        assert_eq!(seq.sum_next_n(0), 0);
        assert_eq!(seq.sum_next_n(1), 2);
        assert_eq!(seq.sum_next_n(9), 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20);

        let mut sum = 0;
        for n in 1..=100 {
            sum += n * 2;
            assert_eq!(EvenNatNumSeq::<u32>::new().sum_next_n(n), sum as u32);
        }
    }

    #[test]
    fn even_nat_num_w0_seq_primitive_types() {
        //! Test that the [EvenNatNumW0Seq] works with different primitive integer types.

        // unsigned types
        assert_eq!(
            EvenNatNumW0Seq::<u8>::new().take(11).collect::<Vec<_>>(),
            (0u8..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0Seq::<u16>::new().take(11).collect::<Vec<_>>(),
            (0u16..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0Seq::<u32>::new().take(11).collect::<Vec<_>>(),
            (0u32..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0Seq::<u64>::new().take(11).collect::<Vec<_>>(),
            (0u64..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0Seq::<u128>::new().take(11).collect::<Vec<_>>(),
            (0u128..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0Seq::<usize>::new().take(11).collect::<Vec<_>>(),
            (0usize..=20).step_by(2).collect::<Vec<_>>()
        );

        // signed types
        assert_eq!(
            EvenNatNumW0Seq::<i8>::new().take(11).collect::<Vec<_>>(),
            (0i8..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0Seq::<i16>::new().take(11).collect::<Vec<_>>(),
            (0i16..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0Seq::<i32>::new().take(11).collect::<Vec<_>>(),
            (0i32..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0Seq::<i64>::new().take(11).collect::<Vec<_>>(),
            (0i64..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0Seq::<i128>::new().take(11).collect::<Vec<_>>(),
            (0i128..=20).step_by(2).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0Seq::<isize>::new().take(11).collect::<Vec<_>>(),
            (0isize..=20).step_by(2).collect::<Vec<_>>()
        );
    }

    #[test]
    fn even_nat_num_w0_seq_verify() {
        //! Test that the [EvenNatNumW0Seq] generates the correct sequence.

        let seq = EvenNatNumW0Seq::<u32>::new();
        assert_eq!(
            seq.take(11).collect::<Vec<u32>>(),
            vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
        );
    }

    #[test]
    fn even_nat_num_w0_seq_nth() {
        //! Test the [EvenNatNumW0Seq::nth] method for various values of `n`.

        for n in 0..100 {
            assert_eq!(
                EvenNatNumW0Seq::<i32>::new().nth(n).unwrap(),
                (n * 2) as i32
            );
        }
    }

    #[test]
    fn even_nat_num_w0_seq_sum_next_n() {
        //! Test the [EvenNatNumW0Seq::sum_next_n] method for a specific number of terms.

        let mut seq = EvenNatNumW0Seq::<u32>::new();
        assert_eq!(seq.sum_next_n(0), 0);
        assert_eq!(seq.sum_next_n(1), 0);
        assert_eq!(
            seq.sum_next_n(10),
            2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20
        );

        let mut sum = 0;
        for n in 1..=100 {
            sum += (n - 1) * 2;
            assert_eq!(EvenNatNumW0Seq::<u32>::new().sum_next_n(n), sum as u32);
        }
    }

    #[test]
    fn nat_num_sq_seq_primitive_types() {
        //! Test that the [NatNumSqSeq] works with different primitive integer types.

        // unsigned types
        assert_eq!(
            NatNumSqSeq::<u8>::new().take(10).collect::<Vec<_>>(),
            (1u8..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSqSeq::<u16>::new().take(10).collect::<Vec<_>>(),
            (1u16..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSqSeq::<u32>::new().take(10).collect::<Vec<_>>(),
            (1u32..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSqSeq::<u64>::new().take(10).collect::<Vec<_>>(),
            (1u64..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSqSeq::<u128>::new().take(10).collect::<Vec<_>>(),
            (1u128..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSqSeq::<usize>::new().take(10).collect::<Vec<_>>(),
            (1usize..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );

        // signed types
        assert_eq!(
            NatNumSqSeq::<i8>::new().take(10).collect::<Vec<_>>(),
            (1i8..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSqSeq::<i16>::new().take(10).collect::<Vec<_>>(),
            (1i16..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSqSeq::<i32>::new().take(10).collect::<Vec<_>>(),
            (1i32..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSqSeq::<i64>::new().take(10).collect::<Vec<_>>(),
            (1i64..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSqSeq::<i128>::new().take(10).collect::<Vec<_>>(),
            (1i128..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumSqSeq::<isize>::new().take(10).collect::<Vec<_>>(),
            (1isize..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn nat_num_sq_seq_verify() {
        //! Test that the [NatNumSqSeq] generates the correct sequence.

        let seq = NatNumSqSeq::<u32>::new();
        assert_eq!(
            seq.take(10).collect::<Vec<u32>>(),
            vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
        );
    }

    #[test]
    fn nat_num_sq_seq_nth() {
        //! Test the [NatNumSqSeq::nth] method for various values of `n`.

        for n in 0..100 {
            assert_eq!(
                NatNumSqSeq::<i32>::new().nth(n).unwrap(),
                (n + 1).pow(2) as i32
            );
        }
    }

    #[test]
    fn nat_num_sq_seq_sum_next_n() {
        //! Test the [NatNumSqSeq::sum_next_n] method for a specific number of terms.

        let mut seq = NatNumSqSeq::<u32>::new();
        assert_eq!(seq.sum_next_n(0), 0);
        assert_eq!(seq.sum_next_n(1), 1);
        assert_eq!(seq.sum_next_n(9), 4 + 9 + 16 + 25 + 36 + 49 + 64 + 81 + 100);

        let mut sum = 0;
        for n in 1..=100 {
            sum += n.pow(2);
            assert_eq!(NatNumSqSeq::<u32>::new().sum_next_n(n), sum as u32);
        }
    }

    #[test]
    fn nat_num_w0_sq_seq_primitive_types() {
        //! Test that the [NatNumW0SqSeq] works with different primitive integer types

        // unsigned types
        assert_eq!(
            NatNumW0SqSeq::<u8>::new().take(11).collect::<Vec<_>>(),
            (0u8..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0SqSeq::<u16>::new().take(11).collect::<Vec<_>>(),
            (0u16..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0SqSeq::<u32>::new().take(11).collect::<Vec<_>>(),
            (0u32..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0SqSeq::<u64>::new().take(11).collect::<Vec<_>>(),
            (0u64..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0SqSeq::<u128>::new().take(11).collect::<Vec<_>>(),
            (0u128..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0SqSeq::<usize>::new().take(11).collect::<Vec<_>>(),
            (0usize..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );

        // signed types
        assert_eq!(
            NatNumW0SqSeq::<i8>::new().take(11).collect::<Vec<_>>(),
            (0i8..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0SqSeq::<i16>::new().take(11).collect::<Vec<_>>(),
            (0i16..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0SqSeq::<i32>::new().take(11).collect::<Vec<_>>(),
            (0i32..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0SqSeq::<i64>::new().take(11).collect::<Vec<_>>(),
            (0i64..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0SqSeq::<i128>::new().take(11).collect::<Vec<_>>(),
            (0i128..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            NatNumW0SqSeq::<isize>::new().take(11).collect::<Vec<_>>(),
            (0isize..=10).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn nat_num_w0_sq_seq_verify() {
        //! Test that the [NatNumW0SqSeq] generates the correct sequence.

        let seq = NatNumW0SqSeq::<u32>::new();
        assert_eq!(
            seq.take(11).collect::<Vec<u32>>(),
            vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
        );
    }

    #[test]
    fn nat_num_w0_sq_seq_nth() {
        //! Test the [NatNumW0SqSeq::nth] method for various values of `n`.

        for n in 0..100 {
            assert_eq!(NatNumW0SqSeq::<i32>::new().nth(n).unwrap(), n.pow(2) as i32);
        }
    }

    #[test]
    fn nat_num_w0_sq_seq_sum_next_n() {
        //! Test the [NatNumW0SqSeq::sum_next_n] method for a specific number of terms.

        let mut seq = NatNumW0SqSeq::<u32>::new();
        assert_eq!(seq.sum_next_n(0), 0);
        assert_eq!(seq.sum_next_n(1), 0);
        assert_eq!(
            seq.sum_next_n(10),
            1 + 4 + 9 + 16 + 25 + 36 + 49 + 64 + 81 + 100
        );

        let mut sum = 0;
        for n in 1..=100 {
            sum += (n - 1).pow(2);
            assert_eq!(NatNumW0SqSeq::<u32>::new().sum_next_n(n), sum as u32);
        }
    }

    #[test]
    fn odd_nat_num_sq_seq_primitive_types() {
        //! Test that the [OddNatNumSqSeq] works with different primitive integer types.

        // unsigned types
        assert_eq!(
            OddNatNumSqSeq::<u8>::new().take(5).collect::<Vec<_>>(),
            (1u8..=9).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSqSeq::<u16>::new().take(10).collect::<Vec<_>>(),
            (1u16..=19).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSqSeq::<u32>::new().take(10).collect::<Vec<_>>(),
            (1u32..=19).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSqSeq::<u64>::new().take(10).collect::<Vec<_>>(),
            (1u64..=19).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSqSeq::<u128>::new().take(10).collect::<Vec<_>>(),
            (1u128..=19)
                .step_by(2)
                .map(|n| n.pow(2))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSqSeq::<usize>::new().take(10).collect::<Vec<_>>(),
            (1usize..=19)
                .step_by(2)
                .map(|n| n.pow(2))
                .collect::<Vec<_>>()
        );

        // signed types
        assert_eq!(
            OddNatNumSqSeq::<i8>::new().take(5).collect::<Vec<_>>(),
            (1i8..=9).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSqSeq::<i16>::new().take(10).collect::<Vec<_>>(),
            (1i16..=19).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSqSeq::<i32>::new().take(10).collect::<Vec<_>>(),
            (1i32..=19).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSqSeq::<i64>::new().take(10).collect::<Vec<_>>(),
            (1i64..=19).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSqSeq::<i128>::new().take(10).collect::<Vec<_>>(),
            (1i128..=19)
                .step_by(2)
                .map(|n| n.pow(2))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            OddNatNumSqSeq::<isize>::new().take(10).collect::<Vec<_>>(),
            (1isize..=19)
                .step_by(2)
                .map(|n| n.pow(2))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn odd_nat_num_sq_seq_verify() {
        //! Test that the [OddNatNumSqSeq] generates the correct sequence.

        let seq = OddNatNumSqSeq::<u32>::new();
        assert_eq!(
            seq.take(10).collect::<Vec<u32>>(),
            vec![1, 9, 25, 49, 81, 121, 169, 225, 289, 361]
        );
    }

    #[test]
    fn odd_nat_num_sq_seq_nth() {
        //! Test the [OddNatNumSqSeq::nth] method for various values of `n`.

        for n in 0..100 {
            assert_eq!(
                OddNatNumSqSeq::<i32>::new().nth(n).unwrap(),
                (n * 2 + 1).pow(2) as i32
            );
        }
    }

    #[test]
    fn odd_nat_num_sq_seq_sum_next_n() {
        //! Test the [OddNatNumSqSeq::sum_next_n] method for a specific number of terms.

        let mut seq = OddNatNumSqSeq::<u32>::new();
        assert_eq!(seq.sum_next_n(0), 0);
        assert_eq!(seq.sum_next_n(1), 1);
        assert_eq!(
            seq.sum_next_n(9),
            9 + 25 + 49 + 81 + 121 + 169 + 225 + 289 + 361
        );

        let mut sum = 0;
        for n in 1..=100 {
            sum += (n * 2 - 1).pow(2);
            assert_eq!(OddNatNumSqSeq::<u32>::new().sum_next_n(n), sum as u32);
        }
    }

    #[test]
    fn even_nat_num_sq_seq_primitive_types() {
        //! Test that the [EvenNatNumSqSeq] works with different primitive integer types.

        // unsigned types
        assert_eq!(
            EvenNatNumSqSeq::<u8>::new().take(5).collect::<Vec<_>>(),
            (2u8..=10).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSqSeq::<u16>::new().take(10).collect::<Vec<_>>(),
            (2u16..=20).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSqSeq::<u32>::new().take(10).collect::<Vec<_>>(),
            (2u32..=20).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSqSeq::<u64>::new().take(10).collect::<Vec<_>>(),
            (2u64..=20).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSqSeq::<u128>::new().take(10).collect::<Vec<_>>(),
            (2u128..=20)
                .step_by(2)
                .map(|n| n.pow(2))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSqSeq::<usize>::new().take(10).collect::<Vec<_>>(),
            (2usize..=20)
                .step_by(2)
                .map(|n| n.pow(2))
                .collect::<Vec<_>>()
        );

        // signed types
        assert_eq!(
            EvenNatNumSqSeq::<i8>::new().take(5).collect::<Vec<_>>(),
            (2i8..=10).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSqSeq::<i16>::new().take(10).collect::<Vec<_>>(),
            (2i16..=20).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSqSeq::<i32>::new().take(10).collect::<Vec<_>>(),
            (2i32..=20).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSqSeq::<i64>::new().take(10).collect::<Vec<_>>(),
            (2i64..=20).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSqSeq::<i128>::new().take(10).collect::<Vec<_>>(),
            (2i128..=20)
                .step_by(2)
                .map(|n| n.pow(2))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumSqSeq::<isize>::new().take(10).collect::<Vec<_>>(),
            (2isize..=20)
                .step_by(2)
                .map(|n| n.pow(2))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn even_nat_num_sq_seq_verify() {
        //! Test that the [EvenNatNumSqSeq] generates the correct sequence.

        let seq = EvenNatNumSqSeq::<u32>::new();
        assert_eq!(
            seq.take(10).collect::<Vec<u32>>(),
            vec![4, 16, 36, 64, 100, 144, 196, 256, 324, 400]
        );
    }

    #[test]
    fn even_nat_num_sq_seq_nth() {
        //! Test the [EvenNatNumSqSeq::nth] method for various values of `n`.

        for n in 0..100 {
            assert_eq!(
                EvenNatNumSqSeq::<i32>::new().nth(n).unwrap(),
                ((n + 1) * 2).pow(2) as i32
            );
        }
    }

    #[test]
    fn even_nat_num_sq_seq_sum_next_n() {
        //! Test the [EvenNatNumSqSeq::sum_next_n] method for a specific number of terms.

        let mut seq = EvenNatNumSqSeq::<u32>::new();
        assert_eq!(seq.sum_next_n(0), 0);
        assert_eq!(seq.sum_next_n(1), 4);
        assert_eq!(
            seq.sum_next_n(9),
            16 + 36 + 64 + 100 + 144 + 196 + 256 + 324 + 400
        );

        let mut sum = 0;
        for n in 1..=100 {
            sum += (n * 2).pow(2);
            assert_eq!(EvenNatNumSqSeq::<u32>::new().sum_next_n(n), sum as u32);
        }
    }

    #[test]
    fn even_nat_num_w0_sq_seq_primitive_types() {
        //! Test that the [EvenNatNumW0SqSeq] works with different primitive integer types.

        // unsigned types
        assert_eq!(
            EvenNatNumW0SqSeq::<u8>::new().take(6).collect::<Vec<_>>(),
            (0u8..=10).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0SqSeq::<u16>::new().take(11).collect::<Vec<_>>(),
            (0u16..=20).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0SqSeq::<u32>::new().take(11).collect::<Vec<_>>(),
            (0u32..=20).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0SqSeq::<u64>::new().take(11).collect::<Vec<_>>(),
            (0u64..=20).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0SqSeq::<u128>::new()
                .take(11)
                .collect::<Vec<_>>(),
            (0u128..=20)
                .step_by(2)
                .map(|n| n.pow(2))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0SqSeq::<usize>::new()
                .take(11)
                .collect::<Vec<_>>(),
            (0usize..=20)
                .step_by(2)
                .map(|n| n.pow(2))
                .collect::<Vec<_>>()
        );

        // signed types
        assert_eq!(
            EvenNatNumW0SqSeq::<i8>::new().take(6).collect::<Vec<_>>(),
            (0i8..=10).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0SqSeq::<i16>::new().take(11).collect::<Vec<_>>(),
            (0i16..=20).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0SqSeq::<i32>::new().take(11).collect::<Vec<_>>(),
            (0i32..=20).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0SqSeq::<i64>::new().take(11).collect::<Vec<_>>(),
            (0i64..=20).step_by(2).map(|n| n.pow(2)).collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0SqSeq::<i128>::new()
                .take(11)
                .collect::<Vec<_>>(),
            (0i128..=20)
                .step_by(2)
                .map(|n| n.pow(2))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            EvenNatNumW0SqSeq::<isize>::new()
                .take(11)
                .collect::<Vec<_>>(),
            (0isize..=20)
                .step_by(2)
                .map(|n| n.pow(2))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn even_nat_num_w0_sq_seq_verify() {
        //! Test that the [EvenNatNumW0SqSeq] generates the correct sequence.

        let seq = EvenNatNumW0SqSeq::<u32>::new();
        assert_eq!(
            seq.take(11).collect::<Vec<u32>>(),
            vec![0, 4, 16, 36, 64, 100, 144, 196, 256, 324, 400]
        );
    }

    #[test]
    fn even_nat_num_w0_sq_seq_nth() {
        //! Test the [EvenNatNumW0SqSeq::nth] method for various values of `n`.

        for n in 0..100 {
            assert_eq!(
                EvenNatNumW0SqSeq::<i32>::new().nth(n).unwrap(),
                (n * 2).pow(2) as i32
            );
        }
    }

    #[test]
    fn even_nat_num_w0_sq_seq_sum_next_n() {
        //! Test the [EvenNatNumW0SqSeq::sum_next_n] method for a specific number of terms.

        let mut seq = EvenNatNumW0SqSeq::<u32>::new();
        assert_eq!(seq.sum_next_n(0), 0);
        assert_eq!(seq.sum_next_n(1), 0);
        assert_eq!(
            seq.sum_next_n(10),
            4 + 16 + 36 + 64 + 100 + 144 + 196 + 256 + 324 + 400
        );

        let mut sum = 0;
        for n in 1..=100 {
            sum += ((n - 1) * 2).pow(2);
            assert_eq!(EvenNatNumW0SqSeq::<u32>::new().sum_next_n(n), sum as u32);
        }
    }
}
