use crate::Solution;
use malachite::Natural;
use malachite::base::num::conversion::traits::Digits;
use pmath::SimpleContinuedFraction;

problem!(Problem0065, 65, "Convergents of e");

impl Solution for Problem0065 {
    fn solve(&self) -> String {
        // generate the values for the continued fraction (2, 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8, ...)
        let mut frac_vals = vec![2];
        frac_vals.extend((0..99).map(|i| if i % 3 == 1 { 2 * (i / 3 + 1) } else { 1 }));

        // get the 100th convergent and sum its digits
        SimpleContinuedFraction::new(frac_vals, None)
            .convergents()
            .nth(99)
            .unwrap()
            .into_numerator()
            .to_digits_asc(&Natural::from(10_u8))
            .iter()
            .sum::<Natural>()
            .to_string()
    }
}
