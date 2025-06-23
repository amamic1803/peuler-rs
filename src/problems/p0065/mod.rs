use crate::Problem;
use crate::math::ContinuedFraction;
use malachite::Natural;
use malachite::base::num::conversion::traits::Digits;

problem!(Problem0065, 65, "Convergents of e");

impl Problem for Problem0065 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        // generate the values for the continued fraction (2, 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8, ...)
        let mut frac_vals = vec![2];
        frac_vals.extend((0..99).map(|i| if i % 3 == 1 { 2 * (i / 3 + 1) } else { 1 }));

        // get the 100th convergent and sum its digits
        ContinuedFraction::new(frac_vals, None)
            .convergent_n(99)
            .unwrap()
            .into_numerator()
            .to_digits_asc(&Natural::from(10_u8))
            .iter()
            .sum::<Natural>()
            .to_string()
    }
}
