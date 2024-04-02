//! **Problem 23** - *Non-Abundant Sums*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(23, "Non-Abundant Sums", solve)
}

use crate::shared::math::sum_of_proper_divisors;
use std::collections::HashSet;

const MIN_ABUNDANT: u64 = 12;
const UPPER_BOUND: u64 = 28123;

fn solve() -> String {
    let mut result = 0;
    let mut abundant_numbers = Vec::new();

    for n in MIN_ABUNDANT..(UPPER_BOUND + 1) {
        let sum = sum_of_proper_divisors(n);
        if sum > n {
            abundant_numbers.push(n);
        }
    }

    let abundant_numbers_set = abundant_numbers.iter().copied().collect::<HashSet<u64>>();

    'outer: for num in 1..(UPPER_BOUND + 1) {
        let limit = num >> 1;
        for addend in &abundant_numbers {
            if *addend > limit {
                break;
            }
            if abundant_numbers_set.contains(&(num - addend)) {
                continue 'outer;
            }
        }
        result += num;
    }

    result.to_string()
}
