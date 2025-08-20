use crate::Solution;
use pmath::SimpleContinuedFraction;

problem!(Problem0064, 64, "Odd Period Square Roots");

impl Solution for Problem0064 {
    fn solve(&self) -> String {
        const MAX: i64 = 10_000;
        // number of continued fractions with the odd period
        let mut odd_period = 0;

        for n in 2..=MAX {
            if let Some(periodic_part) = SimpleContinuedFraction::from_sqrt(n).periodic()
                && periodic_part.len() % 2 == 1
            {
                odd_period += 1;
            }
        }

        odd_period.to_string()
    }
}
