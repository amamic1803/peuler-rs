use crate::Problem;
use crate::math::ContinuedFraction;

problem!(Problem0064, 64, "Odd Period Square Roots");

impl Problem for Problem0064 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        const MAX: i64 = 10_000;
        // number of continued fractions with odd period
        let mut odd_period = 0;

        for n in 2..=MAX {
            if let Some(periodic_part) = ContinuedFraction::from_sqrt(n).periodic() {
                if periodic_part.len() % 2 == 1 {
                    odd_period += 1;
                }
            }
        }

        odd_period.to_string()
    }
}
