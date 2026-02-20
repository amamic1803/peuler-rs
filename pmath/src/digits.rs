//! Functions for working with digits.

use num_traits::{ConstOne, ConstZero, PrimInt};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;

/// Iterator over the digits of an integer in the given radix.
///
/// Digits are yielded in the least significant to the most significant order.
/// # Example
/// ```
/// use pmath::digits::DigitsIter;
///
/// let mut iter = DigitsIter::new(123, 10);
/// assert_eq!(iter.len(), 3);
/// assert_eq!(iter.collect::<Vec<_>>(), vec![3, 2, 1]);
///
/// iter = DigitsIter::new(0b1101u8, 2);
/// assert_eq!(iter.len(), 4);
/// assert_eq!(iter.collect::<Vec<_>>(), vec![1, 0, 1, 1]);
///
/// iter = DigitsIter::new(0u8, 10);
/// assert_eq!(iter.len(), 1);
/// assert_eq!(iter.collect::<Vec<_>>(), vec![0]);
///
/// iter = DigitsIter::new(1, 10);
/// assert_eq!(iter.next().unwrap(), 1);
/// assert_eq!(iter.len(), 0);
///
/// let mut iter = DigitsIter::new(1234567890u32, 10).rev();
/// assert_eq!(iter.len(), 10);
/// assert_eq!(iter.collect::<Vec<_>>(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DigitsIter<T> {
    num: T,
    radix: T,
    front_weight: T,
}
impl<T: PrimInt + ConstZero + ConstOne> DigitsIter<T> {
    /// Create a new [DigitsIter] for the given integer and radix.
    /// # Arguments
    /// * `num` - The integer to extract digits from.
    /// * `radix` - The radix to use for the digits.
    /// # Panics
    /// * If `radix` is less than 2.
    /// * If `num` is negative.
    /// * If `radix` does not fit in the type `T`.
    /// # Returns
    /// * An iterator over the digits of the integer in the given radix.
    pub fn new<U>(num: T, radix: U) -> Self
    where
        U: PrimInt + ConstOne,
    {
        let u2 = U::ONE + U::ONE;
        if radix < u2 {
            panic!("Radix must be at least 2.");
        }
        let radix = T::from(radix).expect("Radix must fit in the type T.");
        let length = match num.cmp(&T::ZERO) {
            Ordering::Less => panic!("Integer must be non-negative."),
            Ordering::Equal => 1,
            Ordering::Greater => num.to_u128().unwrap().ilog(radix.to_u128().unwrap()) + 1,
        };
        let front_weight = radix.pow(length - 1);
        Self {
            num,
            radix,
            front_weight,
        }
    }
}
impl<T: PrimInt + ConstZero> Iterator for DigitsIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num == T::ZERO && self.front_weight == T::ZERO {
            None
        } else {
            let next_digit = self.num % self.radix;
            self.num = self.num / self.radix;
            self.front_weight = self.front_weight / self.radix;
            Some(next_digit)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = if self.front_weight == T::ZERO {
            0
        } else {
            self.front_weight
                .to_u128()
                .unwrap()
                .ilog(self.radix.to_u128().unwrap()) as usize
                + 1
        };
        (length, Some(length))
    }

    fn count(self) -> usize {
        self.len()
    }
}
impl<T: PrimInt + ConstZero> DoubleEndedIterator for DigitsIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.num == T::ZERO && self.front_weight == T::ZERO {
            None
        } else {
            let next_digit = self.num / self.front_weight;
            self.num = self.num % self.front_weight;
            self.front_weight = self.front_weight / self.radix;
            Some(next_digit)
        }
    }
}
impl<T: PrimInt + ConstZero> ExactSizeIterator for DigitsIter<T> {}

/// Create an iterator over the digits of an integer in the given radix.
///
/// Digits are yielded in the least significant to the most significant order.
///
/// This function is a convenience wrapper around [DigitsIter::new].
/// # Arguments
/// * `n` - The integer to extract digits from.
/// * `radix` - The radix to use for the digits.
/// # Panics
/// * If `n` is negative.
/// * If `radix` is less than 2.
/// * If `radix` does not fit in the type `T`.
/// # Returns
/// * An iterator over the digits of the integer in the given radix.
pub fn digits<T, U>(n: T, radix: U) -> DigitsIter<T>
where
    T: PrimInt + ConstZero + ConstOne,
    U: PrimInt + ConstOne,
{
    DigitsIter::new(n, radix)
}

/// Create an integer from digits.
///
/// Digits are expected in the least significant to the most significant order.
/// # Arguments
/// * `digits` - The digits to convert to an integer.
/// * `radix` - The radix of the integer.
/// # Panics
/// * If `radix` is less than 2.
/// * If `radix` does not fit in the type `V`.
/// * If any digit is negative.
/// * If any digit is greater than or equal to `radix`.
/// # Returns
/// * The integer represented by the digits in the given radix.
/// # Example
/// ```
/// use pmath::digits::{digits, digits_to_int};
///
/// let mut n = digits_to_int([3, 2, 1], 10);
/// assert_eq!(n, 123);
/// n = digits_to_int(digits(123, 10), 10);
/// assert_eq!(n, 123);
/// ```
pub fn digits_to_int<T, U, V, Z>(digits: T, radix: Z) -> V
where
    T: IntoIterator<Item = U>,
    U: Borrow<V>,
    V: PrimInt + ConstZero + ConstOne,
    Z: PrimInt + ConstOne,
{
    let z2 = Z::ONE + Z::ONE;
    if radix < z2 {
        panic!("Radix must be at least 2.");
    }
    let radix = V::from(radix).expect("Radix must fit in the type V.");

    let mut result = V::ZERO;
    let mut base = V::ONE;
    for digit in digits {
        let digit = *digit.borrow();
        if digit < V::ZERO {
            panic!("Digits must be non-negative.");
        } else if digit >= radix {
            panic!("Digits must be less than the radix.");
        }
        result = result + base * digit;
        base = base * radix;
    }
    result
}

/// Check whether an integer is a palindrome.
/// # Arguments
/// * `n` - The integer to check.
/// * `radix` - The radix to use for checking.
/// # Returns
/// * Whether an integer is a palindrome.
/// # Panics
/// * If `n` is negative.
/// * If `radix` is less than 2.
/// * If `radix` does not fit in the type `T`.
/// # Example
/// ```
/// use pmath::digits::is_palindrome;
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
pub fn is_palindrome<T, U>(n: T, radix: U) -> bool
where
    T: PrimInt + ConstZero + ConstOne,
    U: PrimInt + ConstOne,
{
    let mut digits = digits(n, radix);
    let mut last_digit = digits.next();
    loop {
        match last_digit {
            Some(last) => {
                match digits.next_back() {
                    Some(front) => {
                        if last != front {
                            return false; // Mismatch found
                        }
                        last_digit = None;
                    }
                    None => return true, // No more digits to compare
                }
            }
            None => {
                match digits.next() {
                    Some(back) => {
                        last_digit = Some(back);
                    }
                    None => return true, // No more digits to compare
                }
            }
        }
    }
}

/// Check if two integers are permutations of each other.
/// # Arguments
/// * `n` - The first integer.
/// * `m` - The second integer.
/// * `radix` - The radix of the integers.
/// # Returns
/// * Whether the integers are permutations of each other.
/// # Panics
/// * If `n` or `m` is negative.
/// * If `radix` is less than 2.
/// * If `radix` does not fit in the type `T`.
/// * If digits of `n` or `m` do not fit in the `usize` type.
/// # Example
/// ```
/// use pmath::digits::is_permutation;
///
/// // 123 and 321 are permutations
/// assert!(is_permutation(123, 321, 10));
/// // 123 and 3210 are not permutations
/// assert!(!is_permutation(123, 3210, 10));
/// // binary 1101 and 1011 are permutations
/// assert!(is_permutation(0b1101, 0b1011, 2));
/// ```
pub fn is_permutation<T, U>(n: T, m: T, radix: U) -> bool
where
    T: PrimInt + ConstZero + ConstOne,
    U: PrimInt + ConstOne,
{
    thread_local! {
        static SEEN_DIGITS: RefCell<HashMap<usize, i16>> = RefCell::new(HashMap::new());
    }

    SEEN_DIGITS.with(|seen| {
        let mut seen = seen.borrow_mut();
        seen.clear();

        for digit in digits(n, radix) {
            let idx = digit.to_usize().expect("Digit must fit in usize.");
            *seen.entry(idx).or_insert(0) += 1;
        }
        for digit in digits(m, radix) {
            let idx = digit.to_usize().expect("Digit must fit in usize.");
            *seen.entry(idx).or_insert(0) -= 1;
        }
        seen.values().all(|v| *v == 0)
    })
}

/// Reverse an integer by reversing its digits in the given radix.
/// # Arguments
/// * `n` - The integer to reverse.
/// * `radix` - The radix to use for reversing the integer.
/// # Returns
/// * The reversed integer.
/// # Panics
/// * If `n` is negative.
/// * If `radix` is less than 2.
/// * If `radix` does not fit in the type `T`.
/// # Example
/// ```
/// use pmath::digits::reverse;
///
/// // 123 -> 321
/// assert_eq!(reverse(123u16, 10), 321);
/// // 0 -> 0
/// assert_eq!(reverse(0u8, 10), 0);
/// // binary 1101 -> 1011
/// assert_eq!(reverse(0b1101u8, 2), 0b1011);
/// ```
pub fn reverse<T, U>(n: T, radix: U) -> T
where
    T: PrimInt + ConstZero + ConstOne,
    U: PrimInt + ConstZero + ConstOne,
{
    digits_to_int(digits(n, radix).rev(), radix)
}

/// Lowercase character representations of hexadecimal digits.
pub const HEX_DIGITS_LOWER: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Uppercase character representations of hexadecimal digits.
pub const HEX_DIGITS_UPPER: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

#[cfg(test)]
mod tests {
    use super::*;

    // DigitsIter tests

    #[test]
    fn digits_iter_integer_primitive_types() {
        //! Test that the [DigitsIter] works with different primitive integer types.

        // unsigned types
        assert_eq!(
            DigitsIter::new(123u8, 10).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123u16, 10).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123u32, 10).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123u64, 10).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123u128, 10).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123usize, 10).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );

        // signed types
        assert_eq!(
            DigitsIter::new(123i8, 10).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123i16, 10).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123i32, 10).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123i64, 10).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123i128, 10).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123isize, 10).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
    }

    #[test]
    fn digits_iter_radix_primitive_types() {
        //! Test that the [DigitsIter] works with different primitive integer types as radix.

        // unsigned types
        assert_eq!(
            DigitsIter::new(123, 10u8).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123, 10u16).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123, 10u32).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123, 10u64).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123, 10u128).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123, 10usize).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );

        // signed types
        assert_eq!(
            DigitsIter::new(123, 10i8).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123, 10i16).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123, 10i32).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123, 10i64).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123, 10i128).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        assert_eq!(
            DigitsIter::new(123, 10isize).collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
    }

    #[test]
    #[should_panic]
    fn digits_iter_negative_integer() {
        //! Test that the [DigitsIter] panics when given a negative integer.

        DigitsIter::new(-123i32, 10);
    }

    #[test]
    #[should_panic]
    fn digits_iter_invalid_radix() {
        //! Test that the [DigitsIter] panics when given an invalid radix.

        DigitsIter::new(123u32, 1);
    }

    #[test]
    #[should_panic]
    fn digits_iter_radix_too_large() {
        //! Test that the [DigitsIter] panics when given a radix that does not fit in the type.

        DigitsIter::new(123u8, 256);
    }

    #[test]
    fn digits_iter_zero() {
        //! Test that the [DigitsIter] correctly handles zero.

        let mut iter = DigitsIter::new(0u8, 10);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next().unwrap(), 0);
        assert_eq!(iter.next(), None);

        let mut iter = DigitsIter::new(0u8, 100);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), 0);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn digits_iter_single_digit() {
        //! Test that the [DigitsIter] correctly handles single-digit integers.

        let mut iter = DigitsIter::new(7u8, 10);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next().unwrap(), 7);
        assert_eq!(iter.next(), None);

        let mut iter = DigitsIter::new(7u8, 100);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next_back().unwrap(), 7);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn digits_iter_general() {
        //! Test that the [DigitsIter] correctly yields digits in the least significant to most significant order.

        let mut iter = DigitsIter::new(12345u32, 10);
        assert_eq!(iter.next().unwrap(), 5);
        assert_eq!(iter.next().unwrap(), 4);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next(), None);

        let mut iter = DigitsIter::new(0b110101u8, 2);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 0);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 0);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn digits_iter_size() {
        //! Test that the [DigitsIter] correctly reports its size.

        let iter = DigitsIter::new(12345u32, 10);
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.count(), 5);

        let iter = DigitsIter::new(0b110101u8, 2);
        assert_eq!(iter.len(), 6);
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.count(), 6);

        let iter = DigitsIter::new(0u8, 10);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.count(), 1);
    }

    #[test]
    fn digits_iter_double_ended() {
        //! Test that the [DigitsIter] correctly implements the [DoubleEndedIterator] trait.

        let mut iter = DigitsIter::new(12345u32, 10);
        assert_eq!(iter.next().unwrap(), 5);
        assert_eq!(iter.next_back().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 4);
        assert_eq!(iter.next_back().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);

        let mut iter = DigitsIter::new(0b110101u8, 2);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next_back().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 0);
        assert_eq!(iter.next_back().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next_back().unwrap(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);

        let mut iter = DigitsIter::new(0u8, 10);
        assert_eq!(iter.next().unwrap(), 0);
        assert_eq!(iter.next_back(), None);
    }

    // digits function tests
    // - only test input types, since it is just a wrapper around DigitsIter::new

    #[test]
    fn digits_integer_primitive_types() {
        //! Test that the [digits] works with different primitive integer types.

        // unsigned types
        assert_eq!(digits(123u8, 10).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123u16, 10).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123u32, 10).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123u64, 10).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123u128, 10).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123usize, 10).collect::<Vec<_>>(), vec![3, 2, 1]);

        // signed types
        assert_eq!(digits(123i8, 10).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123i16, 10).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123i32, 10).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123i64, 10).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123i128, 10).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123isize, 10).collect::<Vec<_>>(), vec![3, 2, 1]);
    }

    #[test]
    fn digits_radix_primitive_types() {
        //! Test that the [digits] works with different primitive integer types as radix.

        // unsigned types
        assert_eq!(digits(123, 10u8).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123, 10u16).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123, 10u32).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123, 10u64).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123, 10u128).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123, 10usize).collect::<Vec<_>>(), vec![3, 2, 1]);

        // signed types
        assert_eq!(digits(123, 10i8).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123, 10i16).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123, 10i32).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123, 10i64).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123, 10i128).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(digits(123, 10isize).collect::<Vec<_>>(), vec![3, 2, 1]);
    }

    #[test]
    #[should_panic]
    fn digits_negative_integer() {
        //! Test that the [digits] panics when given a negative integer.

        digits(-123i32, 10);
    }

    #[test]
    #[should_panic]
    fn digits_invalid_radix() {
        //! Test that the [digits] panics when given an invalid radix.

        digits(123u32, 1);
    }

    #[test]
    #[should_panic]
    fn digits_radix_too_large() {
        //! Test that the [digits] panics when given a radix that does not fit in the type.

        digits(123u8, 256);
    }

    // digits_to_int function tests

    #[test]
    fn digits_to_int_primitive_types() {
        //! Test that the [digits_to_int] works with different primitive integer types.

        // unsigned types
        assert_eq!(digits_to_int([4u8, 3u8], 10), 34);
        assert_eq!(digits_to_int([4u16, 3u16], 10), 34);
        assert_eq!(digits_to_int([4u32, 3u32], 10), 34);
        assert_eq!(digits_to_int([4u64, 3u64], 10), 34);
        assert_eq!(digits_to_int([4u128, 3u128], 10), 34);
        assert_eq!(digits_to_int([4usize, 3usize], 10), 34);

        // signed types
        assert_eq!(digits_to_int([4i8, 3i8], 10), 34);
        assert_eq!(digits_to_int([4i16, 3i16], 10), 34);
        assert_eq!(digits_to_int([4i32, 3i32], 10), 34);
        assert_eq!(digits_to_int([4i64, 3i64], 10), 34);
        assert_eq!(digits_to_int([4i128, 3i128], 10), 34);
        assert_eq!(digits_to_int([4isize, 3isize], 10), 34);
    }

    #[test]
    #[should_panic]
    fn digits_to_int_invalid_radix() {
        //! Test that the [digits_to_int] panics when given an invalid radix.

        digits_to_int([4u8, 3u8], 1);
    }

    #[test]
    #[should_panic]
    fn digits_to_int_radix_too_large() {
        //! Test that the [digits_to_int] panics when given a radix that does not fit in the type.

        digits_to_int([4i8, 3i8], 128);
    }

    #[test]
    #[should_panic]
    fn digits_to_int_negative_digit() {
        //! Test that the [digits_to_int] panics when given a negative digit.

        digits_to_int([-1i8, 3i8], 10);
    }

    #[test]
    #[should_panic]
    fn digits_to_int_digit_ge_radix() {
        //! Test that the [digits_to_int] panics when given a digit that is greater than or equal to the radix.

        digits_to_int([4i8, 3i8], 4);
    }

    #[test]
    fn digits_to_int_general() {
        //! Test that the [digits_to_int] correctly converts digits to an integer.

        assert_eq!(digits_to_int([3, 2, 1], 10), 123);
        assert_eq!(digits_to_int(digits(123, 10), 10), 123);
        assert_eq!(digits_to_int([1, 0, 1, 1], 2), 0b1101);
        assert_eq!(digits_to_int([0], 10), 0);
        assert_eq!(digits_to_int([7], 10), 7);
        assert_eq!(digits_to_int([0, 0, 1], 10), 100);
        assert_eq!(digits_to_int([1, 0, 0], 10), 1);
    }

    // is_palindrome function tests

    #[test]
    fn is_palindrome_primitive_types() {
        //! Test that the [is_palindrome] works with different primitive integer types.

        // unsigned types
        assert!(is_palindrome(121u8, 10));
        assert!(!is_palindrome(123u8, 10));
        assert!(is_palindrome(121u16, 10));
        assert!(!is_palindrome(123u16, 10));
        assert!(is_palindrome(121u32, 10));
        assert!(!is_palindrome(123u32, 10));
        assert!(is_palindrome(121u64, 10));
        assert!(!is_palindrome(123u64, 10));
        assert!(is_palindrome(121u128, 10));
        assert!(!is_palindrome(123u128, 10));
        assert!(is_palindrome(121usize, 10));
        assert!(!is_palindrome(123usize, 10));

        // signed types
        assert!(is_palindrome(121i8, 10));
        assert!(!is_palindrome(123i8, 10));
        assert!(is_palindrome(121i16, 10));
        assert!(!is_palindrome(123i16, 10));
        assert!(is_palindrome(121i32, 10));
        assert!(!is_palindrome(123i32, 10));
        assert!(is_palindrome(121i64, 10));
        assert!(!is_palindrome(123i64, 10));
        assert!(is_palindrome(121i128, 10));
        assert!(!is_palindrome(123i128, 10));
        assert!(is_palindrome(121isize, 10));
        assert!(!is_palindrome(123isize, 10));
    }

    #[test]
    #[should_panic]
    fn is_palindrome_negative_integer() {
        //! Test that the [is_palindrome] panics when given a negative integer.

        is_palindrome(-121i32, 10);
    }

    #[test]
    #[should_panic]
    fn is_palindrome_invalid_radix() {
        //! Test that the [is_palindrome] panics when given an invalid radix.

        is_palindrome(121u32, 1);
    }

    #[test]
    #[should_panic]
    fn is_palindrome_radix_too_large() {
        //! Test that the [is_palindrome] panics when given a radix that does not fit in the type.

        is_palindrome(121i8, 128);
    }

    #[test]
    fn is_palindrome_general() {
        //! Test that the [is_palindrome] correctly identifies palindromic and non-palindromic integers.

        assert!(is_palindrome(0, 10));
        assert!(is_palindrome(1, 10));
        assert!(is_palindrome(11, 10));
        assert!(is_palindrome(121, 10));
        assert!(is_palindrome(1221, 10));
        assert!(is_palindrome(1234567890987654321u128, 10));
        assert!(is_palindrome(0b11011, 2));
        assert!(is_palindrome(0b1001, 2));
        assert!(!is_palindrome(123, 10));
        assert!(!is_palindrome(0b11010, 2));
        assert!(!is_palindrome(0b01010, 2));
    }

    // is_permutation function tests

    #[test]
    fn is_permutation_primitive_types() {
        //! Test that the [is_permutation] works with different primitive integer types.

        // unsigned types
        assert!(is_permutation(123u8, 231u8, 10));
        assert!(!is_permutation(123u8, 241u8, 10));
        assert!(is_permutation(123u16, 231u16, 10));
        assert!(!is_permutation(123u16, 241u16, 10));
        assert!(is_permutation(123u32, 231u32, 10));
        assert!(!is_permutation(123u32, 241u32, 10));
        assert!(is_permutation(123u64, 231u64, 10));
        assert!(!is_permutation(123u64, 241u64, 10));
        assert!(is_permutation(123u128, 231u128, 10));
        assert!(!is_permutation(123u128, 241u128, 10));
        assert!(is_permutation(123usize, 231usize, 10));
        assert!(!is_permutation(123usize, 241usize, 10));

        // signed types
        assert!(is_permutation(23i8, 32i8, 10));
        assert!(!is_permutation(23i8, 42i8, 10));
        assert!(is_permutation(123i16, 231i16, 10));
        assert!(!is_permutation(123i16, 241i16, 10));
        assert!(is_permutation(123i32, 231i32, 10));
        assert!(!is_permutation(123i32, 241i32, 10));
        assert!(is_permutation(123i64, 231i64, 10));
        assert!(!is_permutation(123i64, 241i64, 10));
        assert!(is_permutation(123i128, 231i128, 10));
        assert!(!is_permutation(123i128, 241i128, 10));
        assert!(is_permutation(123isize, 231isize, 10));
        assert!(!is_permutation(123isize, 241isize, 10));
    }

    #[test]
    #[should_panic]
    fn is_permutation_negative_integer() {
        //! Test that the [is_permutation] panics when given a negative integer.

        is_permutation(-123i32, 321i32, 10);
    }

    #[test]
    #[should_panic]
    fn is_permutation_invalid_radix() {
        //! Test that the [is_permutation] panics when given an invalid radix.

        is_permutation(123u32, 321u32, 1);
    }

    #[test]
    #[should_panic]
    fn is_permutation_radix_too_large() {
        //! Test that the [is_permutation] panics when given a radix that does not fit in the type.

        is_permutation(12i8, 12i8, 128);
    }

    #[test]
    #[should_panic]
    fn is_permutation_digit_too_large() {
        //! Test that the [is_permutation] panics when given digits that do not fit in the type.

        if size_of::<usize>() < 128 {
            // if usize is smaller than 128 bits, test using u128
            let large_digit = usize::MAX as u128 + 1;
            let large_radix = large_digit + 1;
            is_permutation(large_digit, large_digit, large_radix);
        } else {
            // otherwise just raise the panic directly, since the test would be meaningless
            panic!("is_permutation_digit_too_large");
        }
    }

    #[test]
    fn is_permutation_general() {
        //! Test that the [is_permutation] correctly identifies permutations and non-permutations.

        assert!(is_permutation(123, 321, 10));
        assert!(!is_permutation(123, 421, 10));
        assert!(is_permutation(0b1101, 0b1011, 2));
        assert!(!is_permutation(0b1101, 0b1001, 2));
        assert!(is_permutation(0, 0, 10));
        assert!(is_permutation(1234567890u64, 9876543210, 10));
        assert!(!is_permutation(0b100, 0b010, 10));
    }

    // reverse function tests

    #[test]
    fn reverse_primitive_types() {
        //! Test that the [reverse] works with different primitive integer types.

        // unsigned types
        assert_eq!(reverse(23u8, 10), 32);
        assert_eq!(reverse(123u16, 10), 321);
        assert_eq!(reverse(123u32, 10), 321);
        assert_eq!(reverse(123u64, 10), 321);
        assert_eq!(reverse(123u128, 10), 321);
        assert_eq!(reverse(123usize, 10), 321);

        // signed types
        assert_eq!(reverse(23i8, 10), 32);
        assert_eq!(reverse(123i16, 10), 321);
        assert_eq!(reverse(123i32, 10), 321);
        assert_eq!(reverse(123i64, 10), 321);
        assert_eq!(reverse(123i128, 10), 321);
        assert_eq!(reverse(123isize, 10), 321);
    }

    #[test]
    #[should_panic]
    fn reverse_negative_integer() {
        //! Test that the [reverse] panics when given a negative integer.

        reverse(-123, 10);
    }

    #[test]
    #[should_panic]
    fn reverse_invalid_radix() {
        //! Test that the [reverse] panics when given an invalid radix.

        reverse(123, 1);
    }

    #[test]
    #[should_panic]
    fn reverse_radix_too_large() {
        //! Test that the [reverse] panics when given a radix that does not fit in the type.

        reverse(123i8, 128);
    }

    #[test]
    fn reverse_general() {
        //! Test that the [reverse] correctly reverses integers.

        assert_eq!(reverse(123, 10), 321);
        assert_eq!(reverse(0, 10), 0);
        assert_eq!(reverse(7, 10), 7);
        assert_eq!(reverse(100, 10), 1);
        assert_eq!(reverse(0b1101, 2), 0b1011);
        assert_eq!(reverse(0b01011, 2), 0b1101);
        assert_eq!(reverse(0xabc, 16), 0xcba);
    }
}
