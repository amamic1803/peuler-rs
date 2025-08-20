use crate::Solution;
use pmath::primes::sieve_of_eratosthenes;

problem!(Problem0010, 10, "Summation of Primes");

impl Solution for Problem0010 {
    fn solve(&self) -> String {
        const LIMIT: u64 = 2_000_000;
        sieve_of_eratosthenes(LIMIT - 1)
            .into_iter()
            .sum::<u64>()
            .to_string()
    }
}
