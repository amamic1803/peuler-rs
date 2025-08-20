use crate::Solution;
use pmath::phi_0_to_n;

problem!(Problem0072, 72, "Counting Fractions");

impl Solution for Problem0072 {
    fn solve(&self) -> String {
        // this is Farey sequence
        // the number of elements in the Farey sequence F(n) is given as:
        // F(n) = 1 + Σ(φ(i)) for i = 1 to n
        let farey_elements = 1 + phi_0_to_n(1_000_000).into_iter().sum::<u64>();

        // the problem excludes 0 and 1, so subtract 2 and return a result
        (farey_elements - 2).to_string()
    }
}
