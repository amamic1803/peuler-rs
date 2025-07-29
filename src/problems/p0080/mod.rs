use crate::Problem;
use crate::math::SimpleContinuedFraction;
use malachite::Natural;
use malachite::base::num::arithmetic::traits::Pow;

problem!(Problem0080, 80, "Square Root Digital Expansion");

impl Problem for Problem0080 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let mut sum = 0;
        let limit = Natural::from(10u32).pow(100);
        for n in 1..=100 {
            let cf = SimpleContinuedFraction::from_sqrt(n);
            let mut last_den = Natural::const_from(1);
            for c in cf.convergents() {
                if &last_den * c.denominator_ref() > limit {
                    let mut i = 0;
                    for d in c.digits(&Natural::const_from(10)).0 {
                        i += 1;
                        sum += i32::try_from(&d).unwrap();
                    }
                    for d in c.digits(&Natural::const_from(10)).1.take(100 - i) {
                        sum += i32::try_from(&d).unwrap();
                    }
                    break;
                } else {
                    last_den = c.into_denominator();
                }
            }
        }
        sum.to_string()
    }
}
