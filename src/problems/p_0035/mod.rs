//! **Problem 35** - *Circular Primes*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        35,
        "Circular Primes",
        solve,
    )
}


use crate::shared::math::{digits, sieve_of_eratosthenes};
use std::collections::HashMap;

fn solve() -> String {
    // generate primes below 1,000,000 using sieve of eratosthenes
    let mut primes = sieve_of_eratosthenes(1_000_000 - 1);

    // remove all primes with digits 0, 2, 4, 5, 6, 8
    // we do that because when we rotate a number with one of those digits
    // when that digit gets to the end of the number, the number will be divisible by 2 or 5
    // and therefore not prime
    primes.retain(|&n| digits(n).all(|digit| !matches!(digit, 0 | 2 | 4 | 5 | 6 | 8)));

    // since we removed all primes with digits 0, 2, 4, 5, 6, 8 we also removed number 2 and 5
    // but these are circular primes so we add them back (because their rotations are themselves)
    primes.insert(0, 2);
    primes.insert(2, 5);

    // create a hashmap of primes for faster lookup, and marking circular ones
    let mut primes_map: HashMap<u64, bool> = HashMap::with_capacity(primes.len());
    for prime in &primes {
        primes_map.insert(*prime, false);
    }

    // for each prime, check if it's circular, and mark it, and all its rotations as circular
    let mut rotations: Vec<u64> = Vec::new();
    for mut n in primes {
        let num_len = digits(n).count();
        let mut circular_prime = true;

        rotations.clear();
        rotations.push(n);

        for _ in 0..(num_len - 1) {
            let last_digit = n % 10;
            n /= 10;
            n += last_digit * 10_u64.pow((num_len - 1) as u32);
            rotations.push(n);
            if !primes_map.contains_key(&n) {
                circular_prime = false;
                break;
            }
        }

        if circular_prime {
            for rotation in &rotations {
                primes_map.insert(*rotation, true);
            }
        }
    }

    // count primes marked as circular
    primes_map.into_iter().filter(|(_, primality)| *primality).count().to_string()
}
