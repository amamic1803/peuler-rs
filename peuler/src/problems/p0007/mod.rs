use crate::Solution;
use pmath::primes::{apcf, sieve_of_eratosthenes};

problem!(Problem0007, 7, "10001st Prime");

impl Solution for Problem0007 {
    fn solve(&self) -> String {
        sieve_of_eratosthenes(apcf(10001u64).round() as u64)[10_000].to_string()
    }
}
