use crate::Solution;
use pmath::digits::is_permutation;
use pmath::phi_0_to_n;

problem!(Problem0070, 70, "Totient Permutation");

impl Solution for Problem0070 {
    fn solve(&self) -> String {
        const MAX_N: u64 = 10_000_000; // maximum n value

        let phi_values = phi_0_to_n(MAX_N); // precompute phi values

        // find the minimum n/phi(n) ratio for (n, phi(n)) that are permutations of each other
        let mut min_ratio = f64::INFINITY;
        let mut min_index = 0;
        for n in 2..=MAX_N {
            if is_permutation(n, phi_values[n as usize], 10) {
                let ratio = n as f64 / phi_values[n as usize] as f64;
                if ratio < min_ratio {
                    min_ratio = ratio;
                    min_index = n;
                }
            }
        }

        // return the index (n) of the minimum ratio
        min_index.to_string()
    }
}
