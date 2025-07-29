use crate::Problem;
use crate::math::SimpleContinuedFraction;
use malachite::Natural;
use malachite::base::num::basic::traits::{One, Zero};

problem!(Problem0066, 66, "Diophantine Equation");

impl Problem for Problem0066 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        const MAX: u16 = 1000;

        // this is Pell's equation
        // the solution to Pell's equation, minimizing x, is one of the convergents of the continued fraction of sqrt(d)
        // so we just need to try out convergents of sqrt(d) in increasing order until we find one that satisfies the equation

        let mut max_x = Natural::ZERO;
        let mut max_d = 0;
        for d in 1..MAX {
            let sqrt_continued_fraction = SimpleContinuedFraction::from_sqrt(d as i64);
            for convergent in sqrt_continued_fraction.convergents() {
                if convergent.denominator_ref() * convergent.denominator_ref() * Natural::from(d)
                    + Natural::ONE
                    == convergent.numerator_ref() * convergent.numerator_ref()
                {
                    if convergent.numerator_ref() > &max_x {
                        max_x = convergent.numerator_ref().clone();
                        max_d = d;
                    }
                    break;
                }
            }
        }

        max_d.to_string()
    }
}
