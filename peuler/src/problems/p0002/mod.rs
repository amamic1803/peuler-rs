use crate::Solution;
use pmath::sequences::FibonacciSeq;

problem!(Problem0002, 2, "Even Fibonacci Numbers");

impl Solution for Problem0002 {
    fn solve(&self) -> String {
        let mut sum = 0;
        for i in FibonacciSeq::<i32>::new() {
            if i > 4000000 {
                break;
            }
            if i % 2 == 0 {
                sum += i;
            }
        }
        sum.to_string()
    }
}
