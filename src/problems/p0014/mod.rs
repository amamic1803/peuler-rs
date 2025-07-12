use crate::Problem;
use crate::math::sequences::CollatzSeq;

problem!(Problem0014, 14, "Longest Collatz Sequence");
impl Problem for Problem0014 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        (1u64..1_000_000)
            .max_by_key(|&n| CollatzSeq::new(n).count())
            .unwrap()
            .to_string()
    }
}
