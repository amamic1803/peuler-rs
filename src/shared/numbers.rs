//! Functions for working with prime numbers.


/// Checks if a number is prime.
/// # Arguments
/// * `num` - The number to check.
/// # Returns
/// * `bool` - Whether the number is prime.
/// * `u128` - The smallest divisor if the number is not prime, otherwise 1.
pub fn is_prime(num: u128) -> (bool, u128) {
    assert!(num >= 2, "Number must be greater than or equal to 2.");

    if num == 2 {
        return (true, 0);
    } else if num % 2 == 0 {
        return (false, 2);
    }

    let mut i = 3;
    while i * i <= num {
        if num % i == 0 {
            return (false, i);
        }
        i += 2;
    }

    (true, 1)
}

/// Checks if a number is a palindrome.
/// # Arguments
/// * `num` - The number to check.
/// # Returns
/// * `bool` - Whether the number is a palindrome.
pub fn is_palindrome(num: u128) -> bool {
    num == reverse(num)
}

/// Reverses a number.
/// # Arguments
/// * `num` - The number to reverse.
/// # Returns
/// * `u128` - The reversed number.
pub fn reverse(mut num: u128) -> u128 {
    let mut new_num = 0;
    while num > 0 {
        new_num = new_num * 10 + num % 10;
        num /= 10;
    }
    new_num
}