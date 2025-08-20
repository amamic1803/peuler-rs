use crate::Solution;
use pmath::isqrt;

problem!(Problem0100, 100, "Arranged Probability");

impl Solution for Problem0100 {
    fn solve(&self) -> String {
        // b = number of blue discs
        // t = total number of discs
        // t > 10^12
        // b/t * (b-1)/(t-1) = 1/2
        // 2b^2 - 2b - t^2 + t = 0
        // b = (1 + sqrt(2t^2 - 2t + 1)) / 2
        // c^2 = 2t^2 - 2t + 1
        // we want b, t to be integers, therefore c must be an odd number
        // solving for t, we get
        // t = (1 + sqrt(2c^2 - 1)) / 2
        // d^2 = 2c^2 - 1
        // d also must be odd, but that is always true for odd c
        // now we have diophantine equation
        // d^2 - 2c^2 = -1
        // this is called negative Pell's equation (x^2 - Dy^2 = -1)
        // fundamental solution can be found in the same way as positive Pell's equation,
        // by trying out convergents of continued fraction of sqrt(2)
        // here it is obvious that the fundamental solution is (1, 1)
        // since t > 10^12, c^2 > (2*10^24 - 2*10^12 + 1), c > sqrt(2*10^24 - 2*10^12 + 1)
        // now we just need to find the solution to negative Pell's equation for which
        // c > sqrt(2*10^24 - 2*10^12 + 1) and c is odd
        // then we can use that c to find
        // b = (1 + c) / 2
        // to find other solutions to negative Pell's equation, besides the fundamental solution,
        // we can use the following recurrence relations
        // x_(n+1) = x_n * (x_1^2 + D * y_1^2) + y_n * 2 * D * x_1 * y_1
        // y_(n+1) = x_n * 2 * x_1 * y_1 + y_n * (x_1^2 + D * y_1^2)
        // where (x_1, y_1) is the fundamental solution
        // plugging in our values, we get
        // d_1 = 1, c_1 = 1
        // d_(n+1) = 3*d_n + 4*c_n
        // c_(n+1) = 2*d_n + 3*c_n

        // find minimum c
        // 2*10^24 - 2*10^12 + 1 is not the perfect square, therefore we can calculate the minimum c
        // by taking the integer square root of 2*10^24 - 2*10^12 + 1 and adding 1
        let min_c = u64::try_from(isqrt(2 * 10u128.pow(24) - 2 * 10_u128.pow(12) + 1) + 1)
            .expect("Overflow");

        // set initial values for d and c (fundamental solution)
        let mut d = 1;
        let mut c = 1;

        // find c that is greater than min_c or equal to min_c and odd
        while c < min_c || c % 2 == 0 {
            (d, c) = (3 * d + 4 * c, 2 * d + 3 * c);
        }

        // calculate b
        let b = c.div_ceil(2);

        // return result
        b.to_string()
    }
}
