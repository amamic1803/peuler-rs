use crate::Problem;
use malachite::Natural;
use malachite::base::num::arithmetic::traits::PowerOf2;
use malachite::base::num::conversion::traits::Digits;

problem!(Problem0016, 16, "Power Digit Sum");

impl Problem for Problem0016 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        Natural::power_of_2(1000)
            .to_digits_desc(&10)
            .into_iter()
            .sum::<u64>()
            .to_string()
    }
}

// this is a slower, pen and paper style solution
/*
fn solve_manual() -> String {
    let mut working_num: Vec<u64> = vec![1];
    let mut transfer: u64 = 0;
    for _ in 1..=1000 {
        for j in (0..working_num.len()).rev() {
            let working_product: u64 = (working_num[j] * 2) + transfer;
            working_num[j] = working_product % 10;
            transfer = working_product / 10;
        }
        while transfer != 0 {
            working_num.insert(0, transfer % 10);
            transfer /= 10;
        }
    }


    let mut sum: u64 = 0;
    for i in &working_num {
        sum += i;
    }

    sum.to_string()
}
*/
