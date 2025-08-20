use crate::Solution;
use pmath::lcm_multiple;

problem!(Problem0005, 5, "Smallest Multiple");

impl Solution for Problem0005 {
    fn solve(&self) -> String {
        lcm_multiple(1u32..=20).to_string()
    }
}
