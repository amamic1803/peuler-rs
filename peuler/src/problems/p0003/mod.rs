use crate::Solution;
use pmath::factors::prime_factors;

problem!(Problem0003, 3, "Largest Prime Factor");

impl Solution for Problem0003 {
    fn solve(&self) -> String {
        const TARGET: u64 = 600851475143;
        prime_factors(TARGET).max().unwrap().to_string()
    }
}
