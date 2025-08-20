use crate::Solution;
use malachite::Natural;
use malachite::base::num::arithmetic::traits::Pow;
use malachite::rational::Rational;
use pmath::SimpleContinuedFraction;

problem!(Problem0080, 80, "Square Root Digital Expansion");

impl Solution for Problem0080 {
    fn solve(&self) -> String {
        // this is solved using simple continued fractions
        // for every integer n from 1 to 100, we create the continued fraction for sqrt(n)
        // if the continued fraction is periodic has a periodic part,
        // it means that n is not a perfect square
        // and then we just need to find the large enough convergent that will give
        // wanted precision (100 digits after the decimal point, actually we don't need 100 digits,
        // since digits before the decimal point are also taken into account,
        // but that will be one digit in this problem so we can ignore that and
        // just calculate 100 digits after the decimal point)
        // to know which convergent is precise enough,
        // we use the property of convergents of continued fractions:
        // abs(x - pk/qk) < 1/(qk * qk+1)
        // we need the error to be smaller than 10^(-100), so:
        // abs(x - pk/qk) < 1/(qk * qk+1) <= 10^(-100)
        // so we need qk * qk+1 >= 10^100
        // that means that we need to find the first convergent where
        // the product of the denominator of the convergent and the denominator of the next convergent
        // is greater than or equal to 10^100

        let limit = Natural::from(10u8).pow(100);
        let mut sum = 0;

        for n in 1..=100 {
            let cf = SimpleContinuedFraction::from_sqrt(n);
            if cf.periodic().is_none() {
                // skip perfect squares
                continue;
            }
            let mut last_convergent = Rational::from(1);
            for c in cf.convergents() {
                if last_convergent.denominator_ref() * c.denominator_ref() >= limit {
                    let (before_decimal, after_decimal) = c.digits(&Natural::const_from(10));
                    let i = before_decimal.len(); // counter for digits before the decimal point
                    for d in before_decimal {
                        sum += i32::try_from(&d).unwrap();
                    }
                    for d in after_decimal.take(100 - i) {
                        sum += i32::try_from(&d).unwrap();
                    }
                    break;
                } else {
                    last_convergent = c;
                }
            }
        }

        sum.to_string()
    }
}
