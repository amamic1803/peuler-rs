use crate::Problem;
use crate::math::sequences::{NaturalNumbersSeq, NaturalNumbersSquaredSeq, Sequence};

problem!(Problem0006, 6, "Sum Square Difference");
impl Problem for Problem0006 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let sum_of_squares = NaturalNumbersSquaredSeq::<i32>::new().sum_next_n(100);
        let square_of_sum = NaturalNumbersSeq::<i32>::new().sum_next_n(100).pow(2);

        sum_of_squares.abs_diff(square_of_sum).to_string()
    }
}
