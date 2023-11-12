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

/// Returns the iterator over the digits of a number.
pub fn digits(n: u64) -> Digits {
    Digits { current: n }
}
pub struct Digits {
    current: u64,
}
impl Iterator for Digits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 0 {
            return None;
        }
        let result = self.current % 10;
        self.current /= 10;
        Some(result)
    }
}

/// Calculates the factorial of a number.
/// # Arguments
/// * `n` - The number to find the factorial of.
/// # Returns
/// * `u64` - The factorial of the number.
pub fn factorial(n: u64) -> u64 {
    (1..(n + 1)).product()
}

/// Finds the greatest common divisor of two numbers.
/// Uses the Euclidean algorithm.
/// # Arguments
/// * `num1` - The first number.
/// * `num2` - The second number.
/// # Returns
/// * `u64` - The greatest common divisor.
pub fn gcd(mut num1: u64, mut num2: u64) -> u64 {
    if num1 < num2 {
        (num1, num2) = (num2, num1);
    }
    while num2 != 0 {
        (num1, num2) = (num2, num1 % num2);
    }
    num1
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

/// Checks if a u128 number is a palindrome.
/// # Arguments
/// * `num` - The number to check.
/// # Returns
/// * `bool` - Whether the number is a palindrome.
pub fn is_palindrome_128(num: u128) -> bool {
    num == reverse_128(num)
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

/// Calculates the zero of a function using Newton's method.
/// Note that this will run forever if the function does not converge to a zero.
/// # Arguments
/// * `x0` - The initial guess.
/// * `precision` - The precision of the answer (the error will be less than this).
/// * `function` - The function to find the zero of.
/// * `derivative` - The derivative of the function.
/// # Returns
/// * `f64` - The zero of the function.
pub fn newtons_method<F, D>(x0: f64, precision: f64, function: F, derivative: D) -> f64
where
    F: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    let mut x = x0;
    let mut prev_x = f64::NEG_INFINITY;

    while (x - prev_x).abs() > precision {
        prev_x = x;
        x = prev_x - function(prev_x) / derivative(prev_x);
    }

    x
}

/// Finds the number of divisors of a number.
/// # Arguments
/// * `n` - The number to find the number of divisors of.
/// # Returns
/// * `u64` - The number of divisors of the number.
pub fn num_of_divisors(n: u64) -> u64 {
    // let n be a natural number
    // we can factorise n
    // n = p1^a1 * p2^a2 * p3^a3 * ... * pn^an
    // (where p1, p2, p3, ..., pn are prime numbers)
    // a function d(n) returns the number of divisors of n
    // obviously, d(n) = (a1 + 1) * (a2 + 1) * (a3 + 1) * ... * (an + 1)

    prime_factors(n).into_iter().map(|(_, a)| a + 1).product()
}

/// Finds the number of proper divisors of a number.
/// Proper divisors are all divisors of a number except the number itself.
/// # Arguments
/// * `n` - The number to find the number of proper divisors of.
/// # Returns
/// * `u64` - The number of proper divisors of the number.
pub fn num_of_proper_divisors(n: u64) -> u64 {
    num_of_divisors(n) - 1
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

/// Finds the prime factors of a number.
/// If the number is 0 or 1, then an empty vector is returned.
/// # Arguments
/// * `x` - The number to find the prime factors of.
/// # Returns
/// * `Vec<[u64; 2]>` - The prime factors of the number. In the form [prime factor, power].
pub fn prime_factors(mut x: u64) -> Vec<(u64, u64)> {
    // calculate primes that are less than or equal to the square root of x
    // these are the only possible prime factors
    let prime_table = sieve_of_eratosthenes((x as f64).sqrt().floor() as u64);
    let mut factors = Vec::new();

    // for every prime factor divide x by that prime factor until it is no longer divisible by that prime factor
    // if x becomes 1 then we have found all prime factors and don't need to check further
    for prime_fact in prime_table {
        let mut fact_info = (prime_fact, 0);

        while x % prime_fact == 0 {
            fact_info.1 += 1;
            x /= prime_fact;
        }

        if fact_info.1 > 0 {
            factors.push(fact_info);
        }

        if x == 1 {
            break;
        }
    }

    // if x is not 1 here, then there are two options
    // 1. x is prime -> we add it to the list of prime factors
    // 2. x is zero -> we do nothing
    if x != 1 && x != 0 {
        factors.push((x, 1));
    }

    factors
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

/// Reverses a u128 number.
/// # Arguments
/// * `num` - The number to reverse.
/// # Returns
/// * `u128` - The reversed number.
pub fn reverse_128(mut num: u128) -> u128 {
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

/// Finds the sum of the divisors of a number.
/// # Arguments
/// * `n` - The number to find the sum of the divisors of.
/// # Returns
/// * `u64` - The sum of the divisors of the number.
pub fn sum_of_divisors(n: u64) -> u64 {
    // let σ(n) be the sum of the divisors of n
    // let p be a prime number
    // then σ(p) = p + 1
    // and σ(p^a) = 1 + p + p^2 + ... + p^a = Σ(k=0, a)p^k = (p^(a + 1) - 1) / (p - 1)
    // we can see that
    // σ(p1^a * p2^b) = 1 + p1 + p1^2 + ... + p1^a + p1*p2 + p1^2*p2 + ... + p1^a*p2 + p1*p2^2 + ... + p1^a*p2^2 + ... + p1^a*p2^b
    // = Σ(k=0, a)p1^k * Σ(k=0, b)p2^k = σ(p1^a) * σ(p2^b)
    // = (p1^(a + 1) - 1) / (p1 - 1) * (p2^(b + 1) - 1) / (p2 - 1)
    // we can also increase this to more than 2 prime factors

    // first we check if n is 0, if it is then we return 0
    if n == 0 {
        0
    } else {
        // if n is not zero then we proceed to find the sum of the divisors
        // note that σ(1) = 1

        // we find the prime factors of n
        // for each we calculate the sum of the divisors of that prime factor
        // and multiply them together

        prime_factors(n).into_iter().map(|(p, a)| (p.pow(a as u32  + 1) - 1) / (p - 1)).product()
    }
}

/// Finds the sum of the proper divisors of a number.
/// Proper divisors are all divisors of a number except the number itself.
/// # Arguments
/// * `n` - The number to find the sum of the proper divisors of.
/// # Returns
/// * `u64` - The sum of the proper divisors of the number.
pub fn sum_of_proper_divisors(n: u64) -> u64 {
    // sum of proper divisors is equal to the sum of all divisors minus the number itself
    sum_of_divisors(n) - n
}
