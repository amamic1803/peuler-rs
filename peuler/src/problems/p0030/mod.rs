use crate::Solution;
use pmath::newtons_method;

problem!(Problem0030, 30, "Digit Fifth Powers");

impl Solution for Problem0030 {
    fn solve(&self) -> String {
        // let x be a number with n digits
        // then x < 10^n
        // the maximum sum of fifth powers of digits of x is 9^5 * n
        // so we need this to be less than 10^n
        // 9^5 * n < 10^n
        // we can find the first n where this is true
        // 9^5 * n = 10^n
        // 10^n - 9^5 * n = 0
        // f(n) = 10^n - 9^5 * n
        // we solve this with newton's method
        // let initial guess be 10

        let max_digits = newtons_method(
            10.0,
            1e-10,
            |n| 10.0_f64.powf(n) - n * (9_u32.pow(5) as f64),
            |n| 10.0_f64.powf(n) * 10.0_f64.ln() - (9_u32.pow(5) as f64),
        )
        .unwrap()
        .ceil() as u32;

        // we now know that the number x can have at most max_digits digits
        // but what value of x should be the upper bound?
        // we know that 9^5 * max_digits is the maximum sum of fifth powers of digits of x
        // so we need only need to check numbers up to 9^5 * max_digits

        let max_num = 9_u32.pow(5) * max_digits;

        // now we can check all numbers up to max_num
        // we start from 10 because single digit numbers are not sums
        let mut result_sum: u32 = 0;
        for n in 10..(max_num + 1) {
            let mut temp_n = n;
            let mut power_sum = 0;

            while temp_n != 0 {
                power_sum += (temp_n % 10).pow(5);
                temp_n /= 10;
            }

            if power_sum == n {
                result_sum += n;
            }
        }

        result_sum.to_string()
    }
}
