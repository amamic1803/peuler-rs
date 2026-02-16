//! Functions related to prime numbers.

use crate::{gcd, newtons_method};
use num_traits::{ConstOne, ConstZero, PrimInt, ToPrimitive};

#[cfg_attr(doc, katexit::katexit)]
/// A prime-counting function.
///
/// Estimates the number of primes less than or equal to $x$.
/// Uses the prime number theorem which states that the number of primes
/// less than or equal to $x$ is approximately $x / \\ln{(x)}$.
/// It is exact for $x < 11$ and guaranteed to be an underestimate for $x >= 11$.
/// # Arguments
/// * `x` - The number to estimate the number of primes less than or equal to.
/// # Returns
/// * The estimated number of primes less than or equal to `x`.
/// # Panics
/// * If `x` cannot be converted to [f64].
/// # Example
/// ```
/// use pmath::primes::pcf;
///
/// assert_eq!(pcf(1), 0.0); // no primes <= 1
/// assert_eq!(pcf(7), 4.0); // 2, 3, 5, 7
/// // exactly 25 primes <= 100, approximation is an underestimate for x >= 11
/// assert!(pcf(100u8) < 25.0);
/// ```
pub fn pcf<T>(x: T) -> f64
where
    T: ToPrimitive,
{
    let x = x.to_f64().expect("Cannot convert x to f64.");
    if x < 2.0 {
        0.0
    } else if x < 3.0 {
        1.0
    } else if x < 5.0 {
        2.0
    } else if x < 7.0 {
        3.0
    } else if x < 11.0 {
        4.0
    } else {
        x / x.ln()
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// An inverse prime-counting function.
///
/// Estimates the number for which the prime-counting function ([pcf]) is approximately $n$.
/// Uses the inverse of the prime number theorem, and the answer is approximated using Newton's method.
/// For $n < 4$ it is calculated exactly using $\\lfloor n \\rfloor$ and
/// guaranteed to be an overestimate for $n >= 4$.
/// # Arguments
/// * `n` - The number to estimate the inverse of the prime-counting function for.
/// # Returns
/// * The estimated inverse of the prime-counting function for `n`.
/// # Panics
/// * If `n` is negative.
/// * If `n` cannot be converted to [f64].
/// # Example
/// ```
/// use pmath::primes::apcf;
///
/// assert_eq!(apcf(0), 0.0);  // zero primes just maps to zero
/// assert_eq!(apcf(2), 3.0);
/// assert!(apcf(25) > 100.0);
/// ```
pub fn apcf<T>(n: T) -> f64
where
    T: ToPrimitive,
{
    let mut n = n.to_f64().expect("Cannot convert n to f64.");
    if n < 0.0 {
        panic!("n must be non-negative.");
    } else if n < 4.0 {
        n = n.floor();
        match n {
            0.0 => 0.0,
            1.0 => 2.0,
            2.0 => 3.0,
            3.0 => 5.0,
            _ => unreachable!(),
        }
    } else {
        let x0 = n + 1.0;
        let precision = 1e-10;
        let function = |x: f64| n * x.ln() - x;
        let derivative = |x: f64| n / x - 1.0;
        newtons_method(x0, precision, function, derivative).unwrap()
    }
}

/// Check if two integers are coprime.
/// # Arguments
/// * `a` - The first integer.
/// * `b` - The second integer.
/// # Returns
/// * Whether the two integers are coprime.
/// # Panics
/// * If either of the integers is negative.
/// # Example
/// ```
/// use pmath::primes::coprime;
///
/// assert!(coprime(7, 20));
/// assert!(!coprime(12, 18));
/// assert!(coprime(15, 28));
/// assert!(!coprime(10, 25));
/// assert!(coprime(1, 1));
/// assert!(coprime(1, 2));
/// assert!(coprime(2, 3));
/// assert!(!coprime(2, 4));
/// ```
pub fn coprime<T>(a: T, b: T) -> bool
where
    T: PrimInt + ConstZero + ConstOne,
{
    gcd(a, b) == T::ONE
}

/// Check if an integer is prime.
/// # Arguments
/// * `n` - The integer to check the primality of.
/// # Returns
/// * `bool` - Whether the integer is prime.
/// * `T` - The smallest divisor if the integer is not prime, otherwise `1`.
/// # Panics
/// * If `n` is less than `2`.
/// * If `n` cannot be converted to [f64].
/// # Example
/// ```
/// use pmath::primes::is_prime;
///
/// // 7 is prime
/// assert_eq!(is_prime(7), (true, 1));
/// // 12 is not prime, the smallest divisor is 2
/// assert_eq!(is_prime(12), (false, 2));
/// ```
pub fn is_prime<T>(n: T) -> (bool, T)
where
    T: PrimInt + ConstZero + ConstOne,
{
    let t2 = T::from(2).unwrap();
    if n < t2 {
        panic!("n must be greater than or equal to 2.");
    }
    let t3 = T::from(3).unwrap();
    let t6 = T::from(6).unwrap();

    if n == t2 || n == t3 {
        (true, T::ONE)
    } else if n % t2 == T::ZERO {
        (false, t2)
    } else if n % t3 == T::ZERO {
        (false, t3)
    } else {
        let upper_bound =
            T::from(n.to_f64().expect("Cannot convert n to f64.").sqrt().floor()).unwrap();
        let mut i = T::from(5).unwrap();
        while i <= upper_bound {
            if n % i == T::ZERO {
                return (false, i);
            } else if n % (i + t2) == T::ZERO {
                return (false, i + t2);
            }
            i = i + t6; // increment by 6
        }

        (true, T::ONE)
    }
}

/// The sieve of Eratosthenes.
///
/// Finds all primes less than or equal to `n`.
/// # Arguments
/// * `n` - The number to find all primes less than or equal to.
/// # Returns
/// * All primes less than or equal to `n`.
/// # Panics
/// * If the sieve requires more elements than can be represented by [usize].
/// # Example
/// ```
/// use pmath::primes::sieve_of_eratosthenes;
///
/// // primes less than or equal to 10: 2, 3, 5, 7
/// assert_eq!(sieve_of_eratosthenes(10), vec![2, 3, 5, 7]);
/// ```
pub fn sieve_of_eratosthenes<T>(n: T) -> Vec<T>
where
    T: PrimInt + ConstOne,
{
    let t2 = T::from(2).unwrap();
    if n < t2 {
        Vec::with_capacity(0)
    } else if n == t2 {
        vec![t2]
    } else {
        let mut results = vec![t2];
        let mut sieve = vec![true; ((n - T::ONE) / t2).to_usize().expect("Sieve too large.")];

        let ind_to_val = |i: usize| ((i as u64) << 1) + 3; // calculate number value from index in sieve
        let val_to_ind = |v: u64| ((v - 3) >> 1) as usize; // calculate index in sieve from number value

        for prime_ind in 0..sieve.len() {
            if sieve[prime_ind] {
                // get prime number value
                let prime_val = ind_to_val(prime_ind);
                let prime_val_2 = prime_val << 1; // prime_val * 2, used to skip even numbers

                // check all multiples of prime number value and mark them as not prime
                // start checking at prime_val^2 (all smaller multiples have already been checked by smaller primes)
                let mut check_val = prime_val * prime_val;
                let mut check_ind = val_to_ind(check_val);
                if check_ind >= sieve.len() {
                    break;
                }

                while check_ind < sieve.len() {
                    sieve[check_ind] = false;
                    // we want check_val to always be odd, prime_val is always odd, so we can just add prime_val * 2
                    // (because if we added 2 odd numbers we would get an even number)
                    check_val += prime_val_2;
                    check_ind = val_to_ind(check_val);
                }
            }
        }

        // convert sieve indices that are true to their corresponding number values and add them to results
        results.extend(sieve.into_iter().enumerate().filter_map(|(i, prime)| {
            if prime {
                Some(T::from(ind_to_val(i)).unwrap())
            } else {
                None
            }
        }));

        // return results
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRIMES_TO_100: [i32; 25] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
        43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97
    ];

    #[test]
    fn pcf_primitive_types() {
        //! Test [pcf] with various numeric types and ensure it does not panic

        // test unsigned integers
        assert_eq!(pcf(10u8), 4.0);
        assert_eq!(pcf(10u16), 4.0);
        assert_eq!(pcf(10u32), 4.0);
        assert_eq!(pcf(10u64), 4.0);
        assert_eq!(pcf(10u128), 4.0);
        assert_eq!(pcf(10usize), 4.0);

        // test signed integers
        assert_eq!(pcf(10i8), 4.0);
        assert_eq!(pcf(10i16), 4.0);
        assert_eq!(pcf(10i32), 4.0);
        assert_eq!(pcf(10i64), 4.0);
        assert_eq!(pcf(10i128), 4.0);
        assert_eq!(pcf(10isize), 4.0);

        // test floating point types
        assert_eq!(pcf(10.0_f32), 4.0);
        assert_eq!(pcf(10.0_f64), 4.0);
    }

    #[test]
    fn pcf_negative_inputs() {
        //! Test [pcf] for negative inputs

        assert_eq!(pcf(-10i32), 0.0);
        assert_eq!(pcf(-1i64), 0.0);
        assert_eq!(pcf(-0.5_f64), 0.0);
        assert_eq!(pcf(-1000000000000i128), 0.0);
    }

    #[test]
    fn pcf_exact_inputs() {
        //! Test [pcf] for inputs where the result is exact according to the piecewise definition

        assert_eq!(pcf(0), 0.0);
        assert_eq!(pcf(1), 0.0);
        assert_eq!(pcf(1.5), 0.0);
        assert_eq!(pcf(2), 1.0);
        assert_eq!(pcf(2.5), 1.0);
        assert_eq!(pcf(3), 2.0);
        assert_eq!(pcf(3.5), 2.0);
        assert_eq!(pcf(4), 2.0);
        assert_eq!(pcf(4.5), 2.0);
        assert_eq!(pcf(5), 3.0);
        assert_eq!(pcf(5.5), 3.0);
        assert_eq!(pcf(6), 3.0);
        assert_eq!(pcf(6.5), 3.0);
        assert_eq!(pcf(7), 4.0);
        assert_eq!(pcf(7.5), 4.0);
        assert_eq!(pcf(8), 4.0);
        assert_eq!(pcf(8.5), 4.0);
        assert_eq!(pcf(9), 4.0);
        assert_eq!(pcf(9.5), 4.0);
        assert_eq!(pcf(10), 4.0);
        assert_eq!(pcf(10.5), 4.0);
    }

    #[test]
    fn pcf_underestimates_for_x_ge_11() {
        //! Test that [pcf] is an underestimate for x >= 11 by comparing to the exact prime count
        //! for integers up to 100

        let mut prime_count = 4;
        for x in 11..=100 {
            if prime_count < PRIMES_TO_100.len() && PRIMES_TO_100[prime_count] == x {
                prime_count += 1;
            }
            let approx = pcf(x);
            assert!(approx < prime_count as f64,
                "pcf({x}) = {approx} should be < exact pi(x) = {prime_count}"
            );
        }
    }

    #[test]
    fn apcf_primitive_types() {
        //! Test [apcf] with various numeric types and ensure it does not panic

        // test unsigned integers
        assert_eq!(apcf(3u8), 5.0);
        assert_eq!(apcf(3u16), 5.0);
        assert_eq!(apcf(3u32), 5.0);
        assert_eq!(apcf(3u64), 5.0);
        assert_eq!(apcf(3u128), 5.0);
        assert_eq!(apcf(3usize), 5.0);

        // test signed integers
        assert_eq!(apcf(3i8), 5.0);
        assert_eq!(apcf(3i16), 5.0);
        assert_eq!(apcf(3i32), 5.0);
        assert_eq!(apcf(3i64), 5.0);
        assert_eq!(apcf(3i128), 5.0);
        assert_eq!(apcf(3isize), 5.0);

        // test floating point types
        assert_eq!(apcf(3.0_f32), 5.0);
        assert_eq!(apcf(3.0_f64), 5.0);
    }

    #[test]
    #[should_panic]
    fn apcf_negative_inputs() {
        //! Test [apcf] for with negative input, which should panic

        apcf(-10i32);
    }

    #[test]
    fn apcf_exact_inputs() {
        //! Test [apcf] for inputs where the result is exact according to the piecewise definition

        assert_eq!(apcf(0), 0.0);
        assert_eq!(apcf(0.9_f64), 0.0);
        assert_eq!(apcf(1), 2.0);
        assert_eq!(apcf(1.2_f64), 2.0);
        assert_eq!(apcf(2), 3.0);
        assert_eq!(apcf(2.7_f64), 3.0);
        assert_eq!(apcf(3), 5.0);
        assert_eq!(apcf(3.9_f64), 5.0);
    }

    #[test]
    fn apcf_overestimates_for_n_ge_4() {
        //! Test that [apcf] is an overestimate for n >= 4 by comparing to the exact inverse prime
        //! count for integers up to 25

         for n in 4..=25 {
             let exact = PRIMES_TO_100[n as usize - 1] as f64;
             let approx = apcf(n);
             assert!(
                 approx > exact,
                 "apcf({n}) = {approx} should be > smallest m with pi(m) = {n}, which is {exact}"
             );
        }
    }

    #[test]
    fn coprime_primitive_types() {
        //! Test [coprime] with various numeric types and ensure it does not panic

        // test unsigned integers
        assert!(coprime(7u8, 20u8));
        assert!(coprime(7u16, 20u16));
        assert!(coprime(7u32, 20u32));
        assert!(coprime(7u64, 20u64));
        assert!(coprime(7u128, 20u128));
        assert!(coprime(7usize, 20usize));
        assert!(!coprime(12u8, 18u8));
        assert!(!coprime(12u16, 18u16));
        assert!(!coprime(12u32, 18u32));
        assert!(!coprime(12u64, 18u64));
        assert!(!coprime(12u128, 18u128));
        assert!(!coprime(12usize, 18usize));

        // test signed integers
        assert!(coprime(7i8, 20i8));
        assert!(coprime(7i16, 20i16));
        assert!(coprime(7i32, 20i32));
        assert!(coprime(7i64, 20i64));
        assert!(coprime(7i128, 20i128));
        assert!(coprime(7isize, 20isize));
        assert!(!coprime(12i8, 18i8));
        assert!(!coprime(12i16, 18i16));
        assert!(!coprime(12i32, 18i32));
        assert!(!coprime(12i64, 18i64));
        assert!(!coprime(12i128, 18i128));
        assert!(!coprime(12isize, 18isize));
    }

    #[test]
    #[should_panic]
    fn coprime_negative_inputs() {
        //! Test [coprime] for with negative input, which should panic

        coprime(-7i32, 20i32);
    }

    #[test]
    fn coprime_zero_inputs() {
        //! Test [coprime] for with zero inputs

        // gcd(a, 0) == |a|, so coprime(a, 0) should be true iff a is 1
        assert!(coprime(1i32, 0i32));
        assert!(!coprime(2i32, 0i32));
        assert!(!coprime(0i32, 0i32)); // gcd(0, 0) is conventionally defined as 0, so not coprime
    }

    #[test]
    fn coprime_symmetry() {
        //! Test that [coprime] is symmetric, i.e. coprime(a, b) == coprime(b, a)

        for i in 0..=20 {
            for j in 0..=20 {
                assert_eq!(
                    coprime(i, j),
                    coprime(j, i),
                    "coprime({i}, {j}) should equal coprime({j}, {i})"
                );
            }
        }
    }

    #[test]
    fn coprime_verify() {
        //! Test that [coprime] returns the expected results

        for i in 1..=20 {
            for j in i..=20 {
                let mut expected = true;
                for k in 2..=i {
                    if i % k == 0 && j % k == 0 {
                        expected = false;
                        break;
                    }
                }
                assert_eq!(
                    coprime(i, j),
                    expected,
                    "coprime({i}, {j}) should be {expected}"
                );
            }
        }
    }

    #[test]
    fn is_prime_primitive_types() {
        //! Test [is_prime] with various numeric types and ensure it does not panic

        // test unsigned integers
        assert_eq!(is_prime(53u8), (true, 1));
        assert_eq!(is_prime(53u16), (true, 1));
        assert_eq!(is_prime(53u32), (true, 1));
        assert_eq!(is_prime(53u64), (true, 1));
        assert_eq!(is_prime(53u128), (true, 1));
        assert_eq!(is_prime(53usize), (true, 1));
        assert_eq!(is_prime(54u8), (false, 2));
        assert_eq!(is_prime(54u16), (false, 2));
        assert_eq!(is_prime(54u32), (false, 2));
        assert_eq!(is_prime(54u64), (false, 2));
        assert_eq!(is_prime(54u128), (false, 2));
        assert_eq!(is_prime(54usize), (false, 2));

        // test signed integers
        assert_eq!(is_prime(53i8), (true, 1));
        assert_eq!(is_prime(53i16), (true, 1));
        assert_eq!(is_prime(53i32), (true, 1));
        assert_eq!(is_prime(53i64), (true, 1));
        assert_eq!(is_prime(53i128), (true, 1));
        assert_eq!(is_prime(53isize), (true, 1));
        assert_eq!(is_prime(54i8), (false, 2));
        assert_eq!(is_prime(54i16), (false, 2));
        assert_eq!(is_prime(54i32), (false, 2));
        assert_eq!(is_prime(54i64), (false, 2));
        assert_eq!(is_prime(54i128), (false, 2));
        assert_eq!(is_prime(54isize), (false, 2));
    }

    #[test]
    #[should_panic]
    fn is_prime_negative_inputs() {
        //! Test [is_prime] for with negative input, which should panic

        is_prime(-7i32);
    }

    #[test]
    #[should_panic]
    fn is_prime_inputs_less_than_2() {
        //! Test [is_prime] for with inputs less than 2, which should panic

        is_prime(0u32);
    }

    #[test]
    fn is_prime_verify() {
        //! Test that [is_prime] returns the expected results for a range of integers

        for n in 2..=100 {
            let (is_p, div) = is_prime(n);
            let expected_is_p = PRIMES_TO_100.contains(&n);
            let expected_div = if expected_is_p { 1 } else {
                // find smallest divisor > 1
                (2..=n).find(|d| n % d == 0).unwrap()
            };
            assert_eq!(
                is_p, expected_is_p,
                "is_prime({n}) = {is_p}, but expected {expected_is_p}"
            );
            assert_eq!(
                div, expected_div,
                "is_prime({n}) divisor = {div}, but expected {expected_div}"
            )
        }
    }

    #[test]
    #[should_panic]
    fn sieve_of_eratosthenes_sieve_too_large() {
        //! Test that [sieve_of_eratosthenes] panics when the sieve would require more elements
        //! than can be represented by [usize]

        if size_of::<usize>() < 128 {
            // usize is less than 128 bits,
            // test the sieve with a large n that would require more
            // than usize::MAX elements in the sieve
            sieve_of_eratosthenes((usize::MAX as u128) * 5);
        } else {
            // just panic to pass the test since sieve cannot require
            // more elements than can be represented by usize
            panic!();
        }
    }

    #[test]
    fn sieve_of_eratosthenes_negative_inputs() {
        //! Test [sieve_of_eratosthenes] for negative inputs, which should return an empty vector

        assert_eq!(sieve_of_eratosthenes(-10i32), Vec::<i32>::new());
        assert_eq!(sieve_of_eratosthenes(-1i64), Vec::<i64>::new());
        assert_eq!(sieve_of_eratosthenes(-1000000000000i128), Vec::<i128>::new());
    }

    #[test]
    fn sieve_of_eratosthenes_verify() {
        //! Test that [sieve_of_eratosthenes] returns the expected primes for various values of n

        for n in 0..=100 {
            let actual = sieve_of_eratosthenes(n);
            let expected = PRIMES_TO_100
                .into_iter()
                .filter(|&p| p <= n)
                .collect::<Vec<_>>();
            assert_eq!(actual, expected,
                "sieve_of_eratosthenes({n}) = {actual:?}, but expected {expected:?}"
            );
        }
    }
}
