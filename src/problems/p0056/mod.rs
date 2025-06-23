use crate::Problem;
use malachite::Natural;
use malachite::base::num::arithmetic::traits::Pow;
use malachite::base::num::conversion::traits::Digits;

problem!(Problem0056, 56, "Powerful Digit Sum");

impl Problem for Problem0056 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let mut max_sum: u32 = 0;
        for a in 1..100_u8 {
            for b in 1..100_u8 {
                let digits_sum = Natural::from(a)
                    .pow(b as u64)
                    .to_digits_desc(&10)
                    .iter()
                    .sum();
                if digits_sum > max_sum {
                    max_sum = digits_sum;
                }
            }
        }
        max_sum.to_string()
    }
}
