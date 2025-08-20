use crate::Solution;
use pmath::factors::sum_of_proper_divisors_0_to_n;

problem!(Problem0021, 21, "Amicable Numbers");

impl Solution for Problem0021 {
    fn solve(&self) -> String {
        const MAX: i32 = 9999;

        // we need to check numbers 1 to 9999 (inclusive)

        // generate the sums of proper divisors for all numbers from 0 to MAX
        let sums = sum_of_proper_divisors_0_to_n(MAX);

        // now for every number, check if it is amicable and add it to the result
        let mut result = 0;
        for (i, &sum) in sums.iter().enumerate().skip(1) {
            let i = i as i32;
            if sum < MAX && (sums[sum as usize] == i) && (i != sum) {
                result += i;
            }
        }
        result.to_string()
    }
}
