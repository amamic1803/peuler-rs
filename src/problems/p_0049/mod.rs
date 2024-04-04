//! **Problem 49** - *Prime Permutations*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(49, "Prime Permutations", solve)
}

use crate::shared::math::{digits, sieve_of_eratosthenes, slice_to_int};
use itertools::Itertools;
use std::collections::BTreeSet;

fn solve() -> String {
    // store all 4-digit primes in a set
    let primes = sieve_of_eratosthenes(9999);
    let mut primes_set = BTreeSet::new();
    for prime in primes {
        if prime > 999 {
            primes_set.insert(prime);
        }
    }

    // vector to store found sequences, there should exist exactly 2 sequences
    let mut found_sequences = Vec::new();

    // logic:
    // take first prime from the set
    // generate its permutations
    // keep only those permutations that are also primes (and remove them from the set)
    // for those numbers, take all combinations of 3
    // and check if they are arithmetic sequences

    let mut prime_perms = Vec::new();
    'outer: while let Some(base_num) = primes_set.pop_first() {
        prime_perms.clear();
        prime_perms.push(base_num);

        // generate permutations
        for perm in digits(base_num).permutations(4) {
            let perm_num = slice_to_int(&perm);

            // try to remove the number from the set
            // if it was in the set, true is returned and the number is added to the vector
            // if it was not in the set, false is returned and the number is not added to the vector
            if primes_set.remove(&perm_num) {
                prime_perms.push(perm_num);
            }
        }

        // sort the permutations so that combinations are increasing for easier arithmetic sequence check
        prime_perms.sort();

        // check for arithmetic sequences
        for (n1, n2, n3) in prime_perms.iter().tuple_combinations() {
            if n2 - n1 == n3 - n2 {
                found_sequences.push([*n1, *n2, *n3]);

                // if 2 sequences are found, break out of the loop
                if found_sequences.len() == 2 {
                    break 'outer;
                }
            }
        }
    }

    // assert that there are exactly 2 sequences
    assert_eq!(found_sequences.len(), 2, "Found {} sequences, expected 2", found_sequences.len());

    // result string
    let mut result = String::new();

    // loop through those 2 sequences and add the non-1487 sequence to the result string
    for seq in found_sequences {
        if !seq.contains(&1487) {
            for num in seq {
                result.push_str(&num.to_string());
            }
        }
    }

    // return
    result
}
