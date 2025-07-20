//! Functions for working with digits.

use std::borrow::Borrow;
use std::cmp::Ordering;
use num_traits::{ConstOne, ConstZero, PrimInt};

/// Iterator over the digits of a number in the given radix.
///
/// Digits are yielded in the order from the least significant to the most significant digit.
/// # Example
/// ```
/// use peuler::math::digits::DigitsIter;
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
/// iter.next();
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
impl<T> DigitsIter<T>
where
    T: PrimInt + ConstZero + ConstOne
{
    /// Creates a new `DigitsIter` for the given number and radix.
    /// # Arguments
    /// * `num` - The number to iterate over.
    /// * `radix` - The radix to use for the digits.
    /// # Panics
    /// * If `radix` is less than 2.
    /// * If `num` is negative.
    /// * If `radix` does not fit in the type `T`.
    /// # Returns
    /// * `DigitsIter<T>` - An iterator over the digits of the number in the given radix.
    pub fn new(num: T, radix: u8) -> Self {
        if radix < 2 {
            panic!("Radix must be at least 2.");
        }
        let radix = T::from(radix).expect("Radix must fit in the type T.");
        let length = match num.cmp(&T::ZERO) {
            Ordering::Less => panic!("Number must be non-negative."),
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
impl<T> Iterator for DigitsIter<T>
where
    T: PrimInt + ConstZero
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num == T::ZERO && self.front_weight == T::ZERO {
            None
        } else {
            let next_digit = self.num % self.radix;
            self.num = self.num / self.radix;
            self.front_weight = self.front_weight / self.radix;
            Some(next_digit.to_u8().unwrap())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = if self.front_weight == T::ZERO {
            0
        } else {
            self.front_weight.to_u128().unwrap().ilog(self.radix.to_u128().unwrap()) as usize + 1
        };
        (length, Some(length))
    }

    fn count(self) -> usize {
        self.len()
    }
}
impl<T> DoubleEndedIterator for DigitsIter<T>
where
    T: PrimInt + ConstZero
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.num == T::ZERO && self.front_weight == T::ZERO {
            None
        } else {
            let next_digit = self.num / self.front_weight;
            self.num = self.num % self.front_weight;
            self.front_weight = self.front_weight / self.radix;
            Some(next_digit.to_u8().unwrap())
        }
    }
}
impl<T> ExactSizeIterator for DigitsIter<T>
where
    T: PrimInt + ConstZero
{}

/// Creates an iterator over the digits of a number in the given radix.
///
/// This function is a convenience wrapper around [DigitsIter::new].
/// # Arguments
/// * `n` - The number to iterate over.
/// * `radix` - The radix to use for the digits.
/// # Panics
/// * If `n` is negative.
/// * If `radix` is less than 2.
/// * If `radix` does not fit in the type `T`.
/// # Returns
/// * An iterator over the digits of the number in the given radix.
pub fn digits<T>(n: T, radix: u8)
    -> impl Iterator<Item = u8> + DoubleEndedIterator + ExactSizeIterator
where
    T: PrimInt + ConstZero + ConstOne
{
    DigitsIter::new(n, radix)
}

/// Creates an integer from digits.
///
/// Digits are interpreted in the least significant to most significant order
/// and can be any type that implements [IntoIterator].
/// # Arguments
/// * `digits` - The digits to convert to an integer,
/// in the least significant to the most significant order.
/// * `radix` - The radix of the number.
/// # Panics
/// * If `radix` is less than 2.
/// * If `radix` does not fit in the type `V`.
/// * If any digit is greater than or equal to `radix`, or negative.
/// # Returns
/// * The integer represented by the digits in the given radix.
/// # Example
/// ```
/// use peuler::math::digits::{digits, digits_to_int};
///
/// let mut n: i32 = digits_to_int([3, 2, 1], 10);
/// assert_eq!(n, 123);
/// n = digits_to_int(digits(123, 10), 10);
/// assert_eq!(n, 123);
/// ```
pub fn digits_to_int<T, U, V>(digits: T, radix: u8) -> V
where
    T: IntoIterator<Item = U>,
    U: Borrow<u8>,
    V: PrimInt + ConstZero + ConstOne
{
    if radix < 2 {
        panic!("Radix must be at least 2.");
    }
    let mut result = V::ZERO;
    let radix = V::from(radix).expect("Radix must fit in the type V.");
    let mut base = V::ONE;
    for digit in digits {
        let digit = V::from(*digit.borrow()).expect("Digit must fit in the type V.");
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

/// Checks whether an integer is a palindrome.
/// # Arguments
/// * `n` - The integer to check.
/// * `radix` - The radix to use for checking.
/// # Returns
/// * `bool` - Whether an integer is a palindrome.
/// # Panics
/// * If `n` is negative.
/// * If `radix` is less than 2.
/// * If `radix` does not fit in the type `T`.
/// # Example
/// ```
/// use peuler::math::digits::is_palindrome;
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
pub fn is_palindrome<T>(n: T, radix: u8) -> bool
where
    T: PrimInt + ConstZero + ConstOne,
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
                    },
                    None => return true, // No more digits to compare
                }
            },
            None => {
                match digits.next() {
                    Some(back) => {
                        last_digit = Some(back);
                    },
                    None => return true, // No more digits to compare
                }
            },
        }
    }
}

/// Checks if two numbers are permutations of each other.
/// # Arguments
/// * `n` - The first number.
/// * `m` - The second number.
/// * `radix` - The radix of the numbers.
/// # Returns
/// * `bool` - Whether the numbers are permutations of each other.
/// # Panics
/// * If `n` or `m` is negative.
/// * If `radix` is less than 2.
/// * If `radix` does not fit in the type `T`.
/// # Example
/// ```
/// use peuler::math::digits::is_permutation;
///
/// // 123 and 321 are permutations
/// assert!(is_permutation(123, 321, 10));
/// // 123 and 3210 are not permutations
/// assert!(!is_permutation(123, 3210, 10));
/// // binary 1101 and 1011 are permutations
/// assert!(is_permutation(0b1101, 0b1011, 2));
/// ```
pub fn is_permutation<T>(n: T, m: T, radix: u8) -> bool
where
    T: PrimInt + ConstZero + ConstOne
{
    let mut seen_digits = [0_i16; 256];

    for digit in digits(n, radix) {
        seen_digits[digit as usize] += 1;
    }
    for digit in digits(m, radix) {
        seen_digits[digit as usize] -= 1;
    }

    seen_digits.into_iter().all(|count| count == 0)
}

/// Reverse an integer.
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
/// use peuler::math::digits::reverse;
///
/// // 123 -> 321
/// assert_eq!(reverse(123u16, 10), 321);
/// // 0 -> 0
/// assert_eq!(reverse(0u8, 10), 0);
/// // binary 1101 -> 1011
/// assert_eq!(reverse(0b1101u8, 2), 0b1011);
/// ```
pub fn reverse<T>(n: T, radix: u8) -> T
where
    T: PrimInt + ConstZero + ConstOne,
{
    digits_to_int(digits(n, radix).rev(), radix)
}
