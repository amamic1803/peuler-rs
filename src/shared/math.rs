//! Mathematical functions.





use std::collections::HashSet;
use std::iter;
use std::mem;
use std::sync::Mutex;
use malachite::num::basic::traits::{Zero, One};
use malachite::Integer;
use malachite::Rational;
use once_cell::sync::Lazy;





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

/// Calculate the integer square root.
/// Slower than casting to f64 and using .sqrt().floor().
/// To be used with big numbers which would lose precision if cast to f64.
/// Uses Newton's method.
pub fn isqrt(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        let mut x0 = 2_u64.pow((n.ilog2() >> 1) + 1);
        let mut x1 = (x0 + n / x0) >> 1;
        while x1 < x0 {
            x0 = x1;
            x1 = (x0 + n / x0) >> 1;
        }
        x0
    }
}

/// Calculate the integer square root of an u128 number.
/// Same as isqrt, but for u128.
pub fn isqrt_128(n: u128) -> u128 {
    if n <= 1 {
        n
    } else {
        let mut x0 = 2_u128.pow((n.ilog2() >> 1) + 1);
        let mut x1 = (x0 + n / x0) >> 1;
        while x1 < x0 {
            x0 = x1;
            x1 = (x0 + n / x0) >> 1;
        }
        x0
    }
}

/// Returns the iterator of the Collatz sequence starting at a number.
pub fn collatz_seq(num: u64) -> impl Iterator<Item=u64> {
    let mut current = num;
    iter::from_fn(move || {
        if current == 1 {
            return None;
        }
        if current % 2 == 0 {
            current >>= 1;
        } else {
            current = 3 * current + 1;
        }
        Some(current)
    })
}

/// Represents a continued fraction.
pub struct ContinuedFraction {
    non_periodic: Vec<i64>,
    periodic: Option<Vec<i64>>,
}
impl ContinuedFraction {
    /// Creates a new continued fraction.
    pub fn new(non_periodic: Vec<i64>, periodic: Option<Vec<i64>>) -> Self {
        Self { non_periodic, periodic }
    }

    /// Creates the continued fraction by taking the square root of a number.
    pub fn from_sqrt(n: i64) -> Self {
        assert!(n >= 0, "Number must be non-negative.");

        // integer square root of n
        let root = (n as f64).sqrt().floor() as i64;

        let non_periodic = vec![root];
        let mut periodic = None;

        // if n is not a perfect square, then find the periodic part of the continued fraction
        if root * root != n {
            // vector for storing the periodic part of the continued fraction
            periodic = Some(Vec::new());

            // set for storing the rational part of the numerator and the denominator
            // used for detecting the period
            let mut set = HashSet::new();

            // rational part of the numerator, it is negative number such that -root < num < 0
            // here it is stored as positive because calculations take into account the negative sign
            let mut num = root;
            // denominator, starts with 1
            let mut denom = 1;

            // calculate next iteration of the continued fraction
            // until the set contains the numerator and the denominator
            // which means the period is found
            while !set.contains(&(num, denom)) {
                set.insert((num, denom));

                denom = (n - num * num) / denom;
                let expanded_val = (num + root) / denom;

                // push the expanded value to the periodic part of the continued fraction
                periodic.as_mut().unwrap().push(expanded_val);

                num = -(num - denom * expanded_val);
            }
        }

        Self { non_periodic, periodic }
    }

    /// Returns the reference to the non-periodic part of the continued fraction.
    pub fn non_periodic(&self) -> &[i64] {
        &self.non_periodic
    }

    /// Returns the reference to the periodic part of the continued fraction.
    pub fn periodic(&self) -> Option<&[i64]> {
        self.periodic.as_deref()
    }

    /// Iterator over the convergents of the continued fraction.
    /// If the continued fraction is finite, then the iterator is also finite.
    /// If the continued fraction is infinite, then the iterator is infinite.
    pub fn convergents(&self) -> impl Iterator<Item=Rational> + '_ {
        let mut prev_num = Integer::ZERO;
        let mut prev_den = Integer::ONE;
        let mut num = Integer::ONE;
        let mut den = Integer::ZERO;
        let mut values = self.non_periodic.iter().chain(self.periodic.iter().flat_map(|v| v.iter().cycle()));

        iter::from_fn(move || {
            let next_value = values.next()?;
            let next_num = Integer::const_from_signed(*next_value) * &num + &prev_num;
            let next_den = Integer::const_from_signed(*next_value) * &den + &prev_den;
            prev_num = mem::replace(&mut num, next_num);
            prev_den = mem::replace(&mut den, next_den);
            Some(Rational::from_integers_ref(&num, &den))
        })
    }

    /// Returns the convergent at index n.
    pub fn convergent_n(&self, n: usize) -> Option<Rational> {
        self.convergents().nth(n)
    }
}

/// Returns the iterator over the digits of a number.
/// Iterates from the least significant digit to the most significant digit.
pub fn digits(n: u64) -> impl Iterator<Item=u8> {
    let mut current = n;
    iter::from_fn(move || {
        if current == 0 {
            return None;
        }
        let result = current % 10;
        current /= 10;
        Some(result as u8)
    })
}

/// Returns the iterator over the digits of a number in reverse order.
/// Iterates from the most significant digit to the least significant digit.
pub fn digits_rev(n: u64) -> impl Iterator<Item=u8> {
    let mut digits_count = n.ilog10() + 1;
    let mut current = reverse(n);
    iter::from_fn(move || {
        if current == 0 && digits_count == 0 {
            return None;
        }
        let result = current % 10;
        current /= 10;
        digits_count -= 1;
        Some(result as u8)
    })
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

/// Checks if two numbers are permutations of each other.
pub fn is_permutation(n: u64, m: u64) -> bool {
    let mut n_digits = [0_u8; 10];
    let mut m_digits = [0_u8; 10];

    for digit in digits(n) {
        n_digits[digit as usize] += 1;
    }
    for digit in digits(m) {
        m_digits[digit as usize] += 1;
    }

    n_digits == m_digits
}

/// Checks if a number is prime.
/// # Arguments
/// * `num` - The number to check.
/// # Returns
/// * `bool` - Whether the number is prime.
/// * `u64` - The smallest divisor if the number is not prime, otherwise 1.
pub fn is_prime(n: u64) -> (bool, u64) {
    assert!(n >= 2, "Number must be greater than or equal to 2.");

    if n == 2 || n == 3 {
        (true, 1)
    } else if n % 2 == 0 {
        (false, 2)
    } else if n % 3 == 0 {
        (false, 3)
    } else {
        for i in (5..=((n as f64).sqrt().floor() as u64)).step_by(6) {
            if n % i == 0 {
                return (false, i);
            } else if n % (i + 2) == 0 {
                return (false, i + 2);
            }
        }

        (true, 1)
    }
}

/// Creates an integer from an iterator over digits.
/// # Arguments
/// * `digits` - The iterator over digits.
/// # Returns
/// * `u64` - The integer.
pub fn iter_to_int<T: IntoIterator<Item=u8>>(digits: T) -> u64 {
    let mut result = 0;
    for digit in digits {
        result *= 10;
        result += digit as u64;
    }
    result
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

/// Calculates multiplicative order.
/// Finds the smallest positive integer k such that a^k ≡ 1 (mod n).
/// a and n must be coprime.
/// # Arguments
/// * `a` - The base.
/// * `n` - The modulus.
/// # Returns
/// * `u64` - The multiplicative order.
/// # Panics
/// Panics if a and n are not coprime.
pub fn ord(a: u64, n: u64) -> u64 {
    assert_eq!(gcd(a, n), 1, "a and n must be coprime.");

    // a^k ≡ 1 (mod n)
    // a^k (mod n) = ((a^(k-1) (mod n)) * a) (mod n)
    // example: 8^2 mod 7 = ((8 mod 7) * 8) mod 7
    // k <= n - 1 (Fermat's little theorem)

    let mut result = 1;
    for k in 1..n {
        result = (result * a) % n;
        if result == 1 {
            return k;
        }
    }

    // since a and n are coprime, multiplicative order must exist
    unreachable!("Multiplicative order not found.");
}

/// Calculates the partition function.
/// Finds the number of ways a number can be written as a sum of positive integers.
/// Uses the recurrence relation p(n) = Σ(k=1, n)(-1)^(k+1) * (p(n - k(3k - 1) / 2) + p(n - k(3k + 1) / 2)).
/// Uses memoization to speed up the calculation.
/// # Arguments
/// * `n` - The number to find the number of partitions of.
/// # Returns
/// * `u64` - The number of partitions of the number.
/// # Example
/// Partitions of 5: {5}, {4, 1}, {3, 2}, {3, 1, 1}, {2, 2, 1}, {2, 1, 1, 1}, {1, 1, 1, 1, 1}
///
/// partition_p(5) = 7
pub fn partition_p(n: u64) -> u64 {
    // get n as usize
    let n = usize::try_from(n).expect("Number too large.");

    // since calculating p(n) also requires calculating p of every number less than n
    // we can just calculate all values and store them in a vector

    // memoization
    static CACHE_VEC: Lazy<Mutex<Vec<u64>>> = Lazy::new(|| {
        Mutex::new(vec![1_u64, 1_u64])
    });
    let mut cache = CACHE_VEC.lock().unwrap();

    // if n is already in the cache vector, then return the value
    if let Some(&value) = cache.get(n) {
        return value;
    }

    while cache.len() <= n {
        // calculate next value and add it to vector

        let curr_n = cache.len();
        let mut next_val = 0;
        for k in 1..=curr_n {
            let left_value = match curr_n.checked_sub((k * (3 * k - 1)) >> 1) {
                Some(ind) => cache[ind],
                None => break,  // larger of the indices is below zero, so any larger k will only be 0, we can break
            };
            let right_value = match curr_n.checked_sub((k * (3 * k + 1)) >> 1) {
                Some(ind) => cache[ind],
                None => 0,
            };
            let value = left_value + right_value;

            if k % 2 == 0 {
                next_val -= value;
            } else {
                next_val += value;
            }
        }

        // push the newly calculated value to the cache vector
        cache.push(next_val);
    }

    // return the value
    cache[n]
}

/// Calculates the number of prime partitions.
/// Finds the number of ways a number can be written as a sum of primes.
/// # Arguments
/// * `n` - The number to find the number of prime partitions of.
/// # Returns
/// * `u64` - The number of prime partitions of the number.
/// # Example
/// Prime partitions of 7: {7}, {5, 2}, {3, 2, 2}
///
/// partition_prime(7) = 3
pub fn partition_prime(n: u64) -> u64 {
    let n = usize::try_from(n).expect("Number too large.");
    let primes = sieve_of_eratosthenes(n as u64);

    let mut dp = vec![0; n + 1];

    // 0 can be represented in 1 way = {} (1 can't be represented as a sum of primes so dp[1] stays 0)
    dp[0] = 1;

    for prime in primes {
        for i in (prime as usize)..=n {
            dp[i] += dp[i - prime as usize];
        }
    }

    dp[n]
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

/// Calculate the Euler's totient function.
pub fn phi(n: u64) -> u64 {
    let mut result = n;
    let mut curr_fact = 0;
    for fact in prime_factors_iter(n) {
        if curr_fact != fact {
            curr_fact = fact;
            result -= result / fact;
        }
    }
    result
}

/// Calculate the Euler's totient function for numbers from 1 to n.
/// Returns a vector of the results, index 0 is unused (set to 0), other indices represent the number.
pub fn phi_1_to_n(n: u64) -> Vec<u64> {
    let mut phi_values: Vec<u64> = (0..=n).collect();

    for i in 2..=n {
        if phi_values[i as usize] == i {
            for j in (i..=n).step_by(i as usize) {
                phi_values[j as usize] -= phi_values[j as usize] / i;
            }
        }
    }

    phi_values
}

/// Represents a point in 2D space.
pub struct Point2D {
    /// The x coordinate.
    pub x: f64,
    /// The y coordinate.
    pub y: f64,
}
impl Point2D {
    /// Creates a new point.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Represents a point in 3D space.
pub struct Point3D {
    /// The x coordinate.
    pub x: f64,
    /// The y coordinate.
    pub y: f64,
    /// The z coordinate.
    pub z: f64,
}
impl Point3D {
    /// Creates a new point.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// Finds the prime factors of a number.
/// If the number is 0 or 1, then an empty vector is returned.
/// # Arguments
/// * `x` - The number to find the prime factors of.
/// # Returns
/// * `Vec<(u64, u64)>` - The prime factors of the number. In the form (prime_factor, power).
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

/// Returns the prime factors as an iterator.
/// If the prime factor appears multiple times, then it will appear multiple times in the iterator.
/// Factors are returned in ascending order.
/// Makes no memory allocations, therefore, it is better for small numbers.
/// For big numbers, prime_factors should be faster because it uses prime sieve.
pub fn prime_factors_iter(mut x: u64) -> impl Iterator<Item=u64> {
    let mut factor = 2;
    iter::from_fn(move || {
        while factor <= x {
            if x % factor == 0 {
                x /= factor;
                return Some(factor);
            }
            match factor {
                2 => factor += 1,
                _ => factor += 2,
            };
        }
        None
    })
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
            let mut results = vec![2];
            let mut sieve = vec![true; (n - 1) as usize / 2];

            let ind_to_val = |i: usize| ((i as u64) << 1) + 3;  // calculate number value from index in sieve
            let val_to_ind = |v: u64| ((v - 3) >> 1) as usize;  // calculate index in sieve from number value

            for prime_ind in 0..sieve.len() {
                if sieve[prime_ind] {
                    // get prime number value
                    let prime_val = ind_to_val(prime_ind);

                    // check all multiples of prime number value and mark them as not prime
                    // start checking at prime_val^2 (all smaller multiples have already been checked by smaller primes)
                    let mut check_val = prime_val * prime_val;
                    let mut check_ind = val_to_ind(check_val);
                    if check_ind >= sieve.len() { break; }

                    while check_ind < sieve.len() {
                        sieve[check_ind] = false;
                        // we want check_val to always be odd, prime_val is always odd so we can just add prime_val * 2
                        // (because if we added 2 odd numbers we would get an even number)
                        check_val += prime_val << 1;
                        check_ind = val_to_ind(check_val);
                    }
                }
            }

            // convert sieve indices that are true to their corresponding number values and add them to results
            results.extend(sieve.into_iter().enumerate().filter_map(|(i, prime)| if prime { Some(ind_to_val(i)) } else { None }));

            // return results
            results
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

/// Converts a slice of digits to an integer.
/// # Arguments
/// * `n` - The slice of digits to convert. Type must be u8.
/// # Returns
/// * `u64` - The integer.
pub fn slice_to_int(n: &[u8]) -> u64 {
    let mut sum: u64 = 0;
    for digit in n {
        sum *= 10;
        sum += *digit as u64;
    }
    sum
}

/// Represents a vector in 2D space.
pub struct Vector2D {
    /// The factor of the x unit vector.
    pub x: f64,
    /// The factor of the y unit vector.
    pub y: f64,
}
impl Vector2D {
    /// Creates a new vector.
    pub fn new(x_factor: f64, y_factor: f64) -> Self {
        Self { x: x_factor, y: y_factor }
    }

    /// Creates a new vector from two points.
    pub fn from_points(point1: Point2D, point2: Point2D) -> Self {
        Self { x: point2.x - point1.x, y: point2.y - point1.y }
    }

    /// Calculates the magnitude of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Calculates the dot product of two vectors.
    pub fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Calculates the angle between two vectors.
    pub fn angle_between(&self, other: &Self) -> f64 {
        (self.dot_product(other) / (self.magnitude() * other.magnitude())).acos()
    }
}

/// Represents a vector in 3D space.
pub struct Vector3D {
    /// The factor of the x unit vector.
    pub x: f64,
    /// The factor of the y unit vector.
    pub y: f64,
    /// The factor of the z unit vector.
    pub z: f64,
}
impl Vector3D {
    /// Creates a new vector.
    pub fn new(x_factor: f64, y_factor: f64, z_factor: f64) -> Self {
        Self { x: x_factor, y: y_factor, z: z_factor }
    }

    /// Creates a new vector from two points.
    pub fn from_points(point1: Point3D, point2: Point3D) -> Self {
        Self { x: point2.x - point1.x, y: point2.y - point1.y, z: point2.z - point1.z }
    }

    /// Calculates the magnitude of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Calculates the dot product of two vectors.
    pub fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates the angle between two vectors.
    pub fn angle_between(&self, other: &Self) -> f64 {
        (self.dot_product(other) / (self.magnitude() * other.magnitude())).acos()
    }

    /// Calculates the cross product of two vectors.
    pub fn cross_product(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}
