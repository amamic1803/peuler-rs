use crate::Solution;
use pmath::partition_prime_0_to_n;

problem!(Problem0077, 77, "Prime Summations");

impl Solution for Problem0077 {
    fn solve(&self) -> String {
        // we don't know the upper bound, but we will just initialize it to 10 and increase
        // it each iteration 10x until we find a solution
        let mut n = 10;
        loop {
            let partition_prime = partition_prime_0_to_n(n);
            for (i, val) in partition_prime.into_iter().enumerate().take(n).skip(n / 10) {
                if val > 5000 {
                    return i.to_string();
                }
            }
            n *= 10;
        }
    }
}
