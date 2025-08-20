use crate::Solution;
use pmath::sequences::CollatzSeq;

problem!(Problem0014, 14, "Longest Collatz Sequence");

impl Solution for Problem0014 {
    fn solve(&self) -> String {
        (1u64..1_000_000)
            .max_by_key(|&n| CollatzSeq::new(n).count())
            .unwrap()
            .to_string()
    }
}
