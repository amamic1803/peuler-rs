//! This module provides functions to generate mathematical sequences.

use num_traits::PrimInt;

/// Trait for sequences that can be iterated over.
/// This is currently just an alias for `Iterator`.
pub trait Sequence: Iterator {}

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
    t0: T,
    t1: T,
    t2: T,
    t3: T,
}
impl<T> CollatzSeq<T>
where
    T: PrimInt,
{
    /// Creates a new Collatz sequence starting at the given number.
    /// # Arguments
    /// * `n` - The number to start the Collatz sequence at.
    /// # Returns
    /// * The iterator over the Collatz sequence.
    pub fn new(n: T) -> Self {
        Self {
            current: n,
            t0: T::from(0).unwrap(),
            t1: T::from(1).unwrap(),
            t2: T::from(2).unwrap(),
            t3: T::from(3).unwrap(),
        }
    }
}
impl<T> Iterator for CollatzSeq<T>
where
    T: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.t0 {
            return None;
        }
        let value = self.current;
        if self.current % self.t2 == self.t0 {
            self.current = self.current / self.t2;
        } else if self.current == self.t1 {
            self.current = self.t0; // ends the sequence
        } else {
            self.current = self.t3 * self.current + self.t1;
        }
        Some(value)
    }
}
impl<T> Sequence for CollatzSeq<T> where T: PrimInt {}

pub struct FibonacciSeq<T> {
    prev: T,
    curr: T,
    index: usize,
}
// TODO: fibonacci (and implement nth function)
