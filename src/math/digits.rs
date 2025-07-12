use std::borrow::Borrow;
use num_traits::{ConstZero, PrimInt, Unsigned};

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
