use crate::Problem;
use malachite::Natural;
use malachite::base::num::arithmetic::traits::FloorLogBase;
use malachite::base::num::basic::traits::{One, Two};
use malachite::rational::Rational;

problem!(Problem0057, 57, "Square Root Convergents");

impl Problem for Problem0057 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let ten_rational = Natural::from(10_u8);
        let mut infinite_fraction = Rational::from_unsigneds(0_u8, 1_u8);
        let mut bigger_numerators = 0;

        for _ in 0..1000 {
            infinite_fraction = Rational::ONE / (Rational::TWO + &infinite_fraction);

            let result = Rational::ONE + &infinite_fraction;

            let numerator_digits = result.numerator_ref().floor_log_base(&ten_rational) + 1;
            let denominator_digits = result.denominator_ref().floor_log_base(&ten_rational) + 1;

            if numerator_digits > denominator_digits {
                bigger_numerators += 1;
            }
        }

        bigger_numerators.to_string()
    }
}
