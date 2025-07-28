use crate::Problem;
use crate::math::partition_p;

problem!(Problem0076, 76, "Counting Summations");

impl Problem for Problem0076 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        // the solution is the number of partitions of 100 minus 1
        // because 100 itself is counted as a partition

        (partition_p(100) - 1).to_string()
    }
}
