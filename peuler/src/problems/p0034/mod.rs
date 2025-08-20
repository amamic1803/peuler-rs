use crate::Solution;
use pmath::{factorial_0_to_n, newtons_method};

problem!(Problem0034, 34, "Digit Factorials");

impl Solution for Problem0034 {
    fn solve(&self) -> String {
        // first we want to find some upper bound for the problem,
        // let n be the number of digits in the number,
        // then the maximum sum of factorials of digits for numbers of size n is
        // 9! * n
        // we want that to be less than
        // 10^n
        // now we have inequality like this
        // 9! * n < 10^n
        // we solve for n using newton's method

        let factorials = factorial_0_to_n(9);

        let n = newtons_method(
            20.0,
            0.01,
            |n| 10.0_f64.powf(n) - factorials[9] as f64 * n,
            |n| 10.0_f64.powf(n) * 10.0_f64.ln() - factorials[9] as f64,
        )
        .unwrap()
        .ceil() as u64;

        // we can calculate the upper bound now
        let upper_bound = factorials[9] * n;

        // now that we know the upper bound, we just start checking numbers
        let mut sum = 0;
        for num in 10..(upper_bound + 1) {
            let mut temp = num;
            let mut digits_sum = 0;
            while temp > 0 {
                digits_sum += factorials[(temp % 10) as usize];
                temp /= 10;
            }
            if digits_sum == num {
                sum += num;
            }
        }

        sum.to_string()
    }
}
