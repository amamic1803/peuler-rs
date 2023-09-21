//! Functions for working with numbers.





/// Inverse of the prime-counting function.
/// Estimates the number for which the prime-counting function is approximately n.
/// Uses the inverse of the prime number theorem.
/// Answer is approximated using Newton's method.
/// It is exact for n <= 3 and guaranteed to be an overestimate for n > 3.
/// # Arguments
/// * `n` - The number to estimate the inverse of the prime-counting function for.
/// # Returns
/// * `f64` - The estimated inverse of the prime-counting function for n.
pub fn apcf(n: u64) -> f64 {
    match n {
        0 => 0.0,
        1 => 2.0,
        2 => 3.0,
        3 => 5.0,
        _ => {
            let a = n as f64;
            let newtons = |x: f64| (a * x * (x.ln() - 1.0)) / (x - a);
            let mut prev_x = f64::NEG_INFINITY;
            let mut x = a + 1.0;
            while (x - prev_x).abs() > 1e-10 {
                prev_x = x;
                x = newtons(x);
            }
            x
        },
    }
}

/// Returns the iterator of the Collatz sequence starting at a number.
pub fn collatz_seq(num: u64) -> CollatzSeq {
    CollatzSeq { current: num }
}
pub struct CollatzSeq {
    current: u64,
}
impl Iterator for CollatzSeq {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 1 {
            return None;
        }
        if self.current % 2 == 0 {
            self.current /= 2;
        } else {
            self.current = 3 * self.current + 1;
        }
        Some(self.current)
    }
}

/// Finds the greatest common divisor of two numbers.
/// Uses the Euclidean algorithm.
/// # Arguments
/// * `num1` - The first number.
/// * `num2` - The second number.
/// # Returns
/// * `u64` - The greatest common divisor.
pub fn gcd(num1: u64, num2: u64) -> u64 {
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
/// * `u64` - The greatest common divisor.
pub fn gcd_multiple(nums: &[u64]) -> u64 {
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
/// * `u64` - The smallest divisor if the number is not prime, otherwise 1.
pub fn is_prime(num: u64) -> (bool, u64) {
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
pub fn is_palindrome(num: u64) -> bool {
    num == reverse(num)
}

/// Finds the least common multiple of two numbers.
/// # Arguments
/// * `num1` - The first number.
/// * `num2` - The second number.
/// # Returns
/// * `u64` - The least common multiple.
pub fn lcm(num1: u64, num2: u64) -> u64 {
    (num1 / gcd(num1, num2)) * num2
}

/// Finds the least common multiple of multiple numbers.
/// # Arguments
/// * `nums` - The numbers.
/// # Returns
/// * `u64` - The least common multiple.
pub fn lcm_multiple(nums: &[u64]) -> u64 {
    assert!(nums.len() > 1, "There must be at least 2 numbers.");
    let mut result = lcm(nums[0], nums[1]);
    for n in nums.iter().skip(2) {
        result = lcm(result, *n);
    }
    result
}

/// Simple prime-counting function.
/// Estimates the number of primes less than or equal to x.
/// Uses the prime number theorem which states that the number of primes less than or equal to x is approximately x / ln(x).
/// It is exact for x <= 10 and guaranteed to be an underestimate for x > 10.
/// # Arguments
/// * `x` - The number to estimate the number of primes less than or equal to.
/// # Returns
/// * `f64` - The estimated number of primes less than or equal to x.
pub fn pcf(x: u64) -> f64 {
    match x {
        0..=1 => 0.0,
        2 => 1.0,
        3..=4 => 2.0,
        5..=6 => 3.0,
        7..=10 => 4.0,
        _ => x as f64 / (x as f64).ln(),
    }
}

/// Exact prime-counting function.
/// Finds the exact number of primes less than or equal to x.
/// Uses the sieve of Eratosthenes.
/// It is not recommended to use this function for large x.
/// # Arguments
/// * `x` - The number to find the number of primes less than or equal to.
/// # Returns
/// * `u64` - The exact number of primes less than or equal to x.
pub fn pcf_exact(x: u64) -> u64 {
    sieve_of_eratosthenes(x).len() as u64
}

/// Reverses a number.
/// # Arguments
/// * `num` - The number to reverse.
/// # Returns
/// * `u64` - The reversed number.
pub fn reverse(mut num: u64) -> u64 {
    let mut new_num = 0;
    while num > 0 {
        new_num = new_num * 10 + num % 10;
        num /= 10;
    }
    new_num
}

/// The sieve of Eratosthenes.
/// Finds all primes less than or equal to n.
/// # Arguments
/// * `n` - The number to find all primes less than or equal to.
/// # Returns
/// * `Vec<u64>` - All primes less than or equal to n.
pub fn sieve_of_eratosthenes(n: u64) -> Vec<u64> {
    match n {
        0..=1 => Vec::with_capacity(0),
        2 => vec![2],
        _ => {
            let mut result = vec![2];
            let mut sieve = vec![true; if n % 2 == 0 { n - 2 } else { n - 1 } as usize / 2];

            let mut prime_index = 0;
            let ind_to_val = |i: usize| 3 + 2 * i as u64;
            let val_to_ind = |v: u64| (v as usize - 3) / 2;

            loop {
                let prime_value = ind_to_val(prime_index);
                let mut current_value = prime_value * prime_value;
                let mut current_position = val_to_ind(current_value);
                if current_position >= sieve.len() {
                    break;
                }
                while current_position < sieve.len() {
                    sieve[current_position] = false;
                    current_value += 2 * prime_value;
                    current_position = val_to_ind(current_value);
                }

                prime_index += 1;
                while prime_index < sieve.len() && !sieve[prime_index] {
                    prime_index += 1;
                }
            }

            result.extend(
                sieve
                    .iter()
                    .enumerate()
                    .filter(|(_, &is_prime)| is_prime)
                    .map(|(i, _)| (i as u64) * 2 + 3)
            );
            result
        },
    }
}

/// Finds the sum of the first n natural numbers.
/// # Arguments
/// * `n` - The number of natural numbers to sum.
/// # Returns
/// * `u64` - The sum of the first n natural numbers.
pub fn sum_n(n: u64) -> u64 {
    n * (n + 1) / 2
}

/// Finds the sum of the first n even natural numbers.
/// # Arguments
/// * `n` - The number of even natural numbers to sum.
/// # Returns
/// * `u64` - The sum of the first n even natural numbers.
pub fn sum_n_even(n: u64) -> u64 {
    n * (n + 1)
}

/// Finds the sum of the squares of the first n even natural numbers.
/// # Arguments
/// * `n` - The number of even natural numbers to sum.
/// # Returns
/// * `u64` - The sum of the squares of the first n even natural numbers.
pub fn sum_n_even_squares(n: u64) -> u64 {
    2 * n * (n + 1) * (2 * n + 1) / 3
}

/// Finds the sum of the first n odd natural numbers.
/// # Arguments
/// * `n` - The number of odd natural numbers to sum.
/// # Returns
/// * `u64` - The sum of the first n odd natural numbers.
pub fn sum_n_odd(n: u64) -> u64 {
    n * n
}

/// Finds the sum of the squares of the first n odd natural numbers.
/// # Arguments
/// * `n` - The number of odd natural numbers to sum.
/// # Returns
/// * `u64` - The sum of the squares of the first n odd natural numbers.
pub fn sum_n_odd_squares(n: u64) -> u64 {
    n * (2 * n + 1) * (2 * n - 1) / 3
}

/// Finds the sum of the squares of the first n natural numbers.
/// # Arguments
/// * `n` - The number of natural numbers to sum.
/// # Returns
/// * `u64` - The sum of the squares of the first n natural numbers.
pub fn sum_n_squares(n: u64) -> u64 {
    n * (n + 1) * (2 * n + 1) / 6
}