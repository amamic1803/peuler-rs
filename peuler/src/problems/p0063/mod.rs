use crate::Solution;
use pmath::newtons_method;

problem!(Problem0063, 63, "Powerful Digit Counts");

impl Solution for Problem0063 {
    fn solve(&self) -> String {
        // let x be natural number that satisfies the requirement
        // let n be the number of digits in x
        // from that we have:
        // 10^(n-1) <= x < 10^n
        // let y be natural number such that x = y^n
        // then we have:
        // 10^(n-1) <= y^n < 10^n
        // let's take care of the right inequality first:
        // y^n < 10^n
        // since n-th root is a monotonic function, we can take the n-th root of both sides:
        // y < 10
        // that is, because y is a natural number, y can only be 1 to 9
        // now let's take care of the left inequality:
        // 10^(n-1) <= y^n
        // if we take the limit of both sides as n approaches infinity, we get:
        // 10^(inf) <= y^(inf)
        // which is obviously not true since y < 10
        // that means that there exists n from which the inequality is not true
        // therefore we need to find the last n for which the inequality is true
        // we can set up a function like this:
        // f(n) = 10^(n-1) - y^n
        // and we find the root of that function using Newton's method
        // initial guess for the root is 100
        // the n we are searching for is the floor of the root
        // (because for every n > root, the inequality is not true)

        // now for given y, there are n numbers n that satisfy the requirement
        // we just have to repeat the process for all y from 1 to 9 and sum up the results

        let mut count = 0;
        for y in 1..10_u8 {
            let n = newtons_method(
                100.0,
                1e-10,
                |n: f64| 10.0_f64.powf(n - 1.0) - (y as f64).powf(n),
                |n: f64| {
                    10.0_f64.powf(n - 1.0) * 10.0_f64.ln() - (y as f64).powf(n) * (y as f64).ln()
                },
            )
            .unwrap()
            .floor() as u64;

            count += n;
        }
        count.to_string()
    }
}
