use crate::Solution;
use pmath::primes::is_prime;

problem!(Problem0046, 46, "Goldbach's Other Conjecture");

impl Solution for Problem0046 {
    fn solve(&self) -> String {
        let mut primes = vec![2];
        let mut current = next_odd_composite(1, &mut primes);

        while satisfies_conjecture(current, &primes) {
            current = next_odd_composite(current, &mut primes);
        }

        current.to_string()
    }
}

/// Get the next odd composite number.
///
/// Store any primes found in `primes`.
/// # Arguments
/// * `current` - The current odd composite number. (can be any odd number, not necessarily composite)
/// * `primes` - A vector of primes.
/// # Returns
/// * The next odd composite number.
fn next_odd_composite(mut current: u64, primes: &mut Vec<u64>) -> u64 {
    current += 2;
    while is_prime(current).0 {
        primes.push(current);
        current += 2;
    }
    current
}

/// Check if a number satisfies the Goldbach's Other Conjecture.
/// # Arguments
/// * `n` - The number to check.
/// * `primes` - A vector of primes smaller than `n`.
/// # Returns
/// * `true` if `n` satisfies the conjecture, `false` otherwise.
fn satisfies_conjecture(n: u64, primes: &[u64]) -> bool {
    for p in primes {
        let maybe_square = (n - p) / 2;
        let maybe_int = (maybe_square as f64).sqrt();
        if (maybe_int - maybe_int.round()).abs() < 10e-8 {
            return true;
        }
    }
    false
}
