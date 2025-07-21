use crate::Problem;
use crate::math::phi_0_to_n;

problem!(Problem0069, 69, "Totient Maximum");

impl Problem for Problem0069 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        const MAX_N: u64 = 1_000_000; // maximum n value
        let phi_values = phi_0_to_n(MAX_N); // precompute phi values

        // find the maximum n/phi(n) ratio
        let mut max_ratio = 0.0;
        let mut max_index = 0;
        for n in 1..=MAX_N {
            let ratio = n as f64 / phi_values[n as usize] as f64;
            if ratio > max_ratio {
                max_ratio = ratio;
                max_index = n;
            }
        }

        // return the index (n) of the maximum ratio
        max_index.to_string()
    }
}
