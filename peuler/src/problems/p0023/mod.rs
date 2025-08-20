use crate::Solution;
use pmath::factors::sum_of_proper_divisors_0_to_n;
use std::collections::HashSet;

problem!(Problem0023, 23, "Non-Abundant Sums");

impl Solution for Problem0023 {
    fn solve(&self) -> String {
        const UPPER_BOUND: u64 = 28123;

        let mut result = 0;

        let mut abundant_numbers = Vec::new();
        for (i, sum) in sum_of_proper_divisors_0_to_n(UPPER_BOUND)
            .into_iter()
            .enumerate()
        {
            if sum > i as u64 {
                abundant_numbers.push(i as u64);
            }
        }

        let abundant_numbers_set = abundant_numbers.iter().copied().collect::<HashSet<u64>>();

        'outer: for num in 1..(UPPER_BOUND + 1) {
            let limit = num / 2;
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
}
