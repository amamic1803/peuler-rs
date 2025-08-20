use crate::Solution;
use pmath::partition_p;

problem!(Problem0076, 76, "Counting Summations");

impl Solution for Problem0076 {
    fn solve(&self) -> String {
        // the solution is the number of partitions of 100 minus 1
        // because 100 itself is counted as a partition

        (partition_p(100) - 1).to_string()
    }
}
