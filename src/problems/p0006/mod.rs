use crate::Problem;
use crate::math::{sum_n, sum_n_squares};

problem!(Problem0006, 6, "Sum Square Difference");
impl Problem for Problem0006 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let sum_of_squares = sum_n_squares(100u32);
        let square_of_sum = sum_n(100u32).pow(2);

        sum_of_squares.abs_diff(square_of_sum).to_string()
    }
}
