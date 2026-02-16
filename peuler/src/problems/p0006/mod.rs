use crate::Solution;
use pmath::sequences::{NatNumSeq, NatNumSqSeq, Sequence};

problem!(Problem0006, 6, "Sum Square Difference");

impl Solution for Problem0006 {
    fn solve(&self) -> String {
        let sum_of_squares = NatNumSqSeq::<i32>::new().sum_next_n(100);
        let square_of_sum = NatNumSeq::<i32>::new().sum_next_n(100).pow(2);

        sum_of_squares.abs_diff(square_of_sum).to_string()
    }
}
