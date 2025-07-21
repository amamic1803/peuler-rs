use crate::Problem;
use crate::math::factors::proper_divisors;
use std::collections::HashSet;

problem!(Problem0023, 23, "Non-Abundant Sums");

impl Problem for Problem0023 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        const MIN_ABUNDANT: u64 = 12;
        const UPPER_BOUND: u64 = 28123;

        let mut result = 0;
        let mut abundant_numbers = Vec::new();

        for n in MIN_ABUNDANT..(UPPER_BOUND + 1) {
            let sum = proper_divisors(n).sum::<u64>();
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
}
