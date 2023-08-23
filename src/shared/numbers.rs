//! Functions for working with numbers.


/// Finds the greatest common divisor of two numbers.
/// Uses the Euclidean algorithm.
/// # Arguments
/// * `num1` - The first number.
/// * `num2` - The second number.
/// # Returns
/// * `u128` - The greatest common divisor.
pub fn gcd(num1: u128, num2: u128) -> u128 {
    let mut nums = if num1 > num2 { [num1, num2] } else { [num2, num1] };
    while nums[1] != 0 {
        (nums[0], nums[1]) = (nums[1], nums[0] % nums[1]);
    }
    nums[0]
}

/// Finds the greatest common divisor of multiple numbers.
/// # Arguments
/// * `nums` - The numbers.
/// # Returns
/// * `u128` - The greatest common divisor.
pub fn gcd_multiple(nums: &[u128]) -> u128 {
    assert!(nums.len() > 1, "There must be at least 2 numbers.");
    let mut result = gcd(nums[0], nums[1]);
    for n in nums.iter().skip(2) {
        result = gcd(result, *n);
    }
    result
}

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

/// Finds the least common multiple of two numbers.
/// # Arguments
/// * `num1` - The first number.
/// * `num2` - The second number.
/// # Returns
/// * `u128` - The least common multiple.
pub fn lcm(num1: u128, num2: u128) -> u128 {
    (num1 / gcd(num1, num2)) * num2
}

/// Finds the least common multiple of multiple numbers.
/// # Arguments
/// * `nums` - The numbers.
/// # Returns
/// * `u128` - The least common multiple.
pub fn lcm_multiple(nums: &[u128]) -> u128 {
    assert!(nums.len() > 1, "There must be at least 2 numbers.");
    let mut result = lcm(nums[0], nums[1]);
    for n in nums.iter().skip(2) {
        result = lcm(result, *n);
    }
    result
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

/// Finds the sum of the first n natural numbers.
/// # Arguments
/// * `n` - The number of natural numbers to sum.
/// # Returns
/// * `u128` - The sum of the first n natural numbers.
pub fn sum_n(n: u128) -> u128 {
    n * (n + 1) / 2
}

/// Finds the sum of the first n even natural numbers.
/// # Arguments
/// * `n` - The number of even natural numbers to sum.
/// # Returns
/// * `u128` - The sum of the first n even natural numbers.
pub fn sum_n_even(n: u128) -> u128 {
    n * (n + 1)
}

/// Finds the sum of the squares of the first n even natural numbers.
/// # Arguments
/// * `n` - The number of even natural numbers to sum.
/// # Returns
/// * `u128` - The sum of the squares of the first n even natural numbers.
pub fn sum_n_even_squares(n: u128) -> u128 {
    2 * n * (n + 1) * (2 * n + 1) / 3
}

/// Finds the sum of the first n odd natural numbers.
/// # Arguments
/// * `n` - The number of odd natural numbers to sum.
/// # Returns
/// * `u128` - The sum of the first n odd natural numbers.
pub fn sum_n_odd(n: u128) -> u128 {
    n * n
}

/// Finds the sum of the squares of the first n odd natural numbers.
/// # Arguments
/// * `n` - The number of odd natural numbers to sum.
/// # Returns
/// * `u128` - The sum of the squares of the first n odd natural numbers.
pub fn sum_n_odd_squares(n: u128) -> u128 {
    n * (2 * n + 1) * (2 * n - 1) / 3
}

/// Finds the sum of the squares of the first n natural numbers.
/// # Arguments
/// * `n` - The number of natural numbers to sum.
/// # Returns
/// * `u128` - The sum of the squares of the first n natural numbers.
pub fn sum_n_squares(n: u128) -> u128 {
    n * (n + 1) * (2 * n + 1) / 6
}