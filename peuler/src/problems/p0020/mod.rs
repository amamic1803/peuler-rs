use crate::Solution;
use malachite::Natural;
use malachite::base::num::arithmetic::traits::Factorial;
use malachite::base::num::conversion::traits::Digits;

problem!(Problem0020, 20, "Factorial Digit Sum");

impl Solution for Problem0020 {
    fn solve(&self) -> String {
        Natural::factorial(100)
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
    for n in 2..=100 {
        let mut factoring: Vec<Vec<u64>> = vec![];
        let mut factoring_num: u64 = n;
        while factoring_num != 0 {
            let mut current_product: Vec<u64> = vec![];
            let mut transfer: u64 = 0;
            for j in (0..working_num.len()).rev() {
                let working_product: u64 = (working_num[j] * (factoring_num % 10)) + transfer;
                current_product.insert(0, working_product % 10);
                transfer = working_product / 10;
            }
            while transfer != 0 {
                current_product.insert(0, transfer % 10);
                transfer /= 10;
            }
            factoring.push(current_product);
            factoring_num /= 10;
        }
        for i in 0..factoring.len() {
            for _  in 0..i {
                factoring[i].push(0)
            }
        }
        let mut longest: usize = 0;
        for i in 0..factoring.len() {
            if factoring[i].len() > longest {
                longest = factoring[i].len();
            }
        }
        for i in 0..factoring.len() {
            for _ in 0..(longest - factoring[i].len()) {
                factoring[i].insert(0, 0);
            }
        }
        let mut result: Vec<u64> = vec![];
        let mut transfer: u64 = 0;
        for j in (0..longest).rev() {
            let mut working_sum: u64 = transfer;
            for i in 0..factoring.len() {
                working_sum += factoring[i][j];
            }
            result.insert(0, working_sum % 10);
            transfer = working_sum / 10;
        }
        while transfer != 0 {
            result.insert(0, transfer % 10);
            transfer /= 10;
        }
        working_num = result;
    }
    let mut sum: u64 = 0;
    for i in &working_num {
        sum += i;
    }
    sum.to_string()
}
*/
