use crate::Problem;
use crate::math::collatz_seq;

problem!(Problem0014, 14, "Longest Collatz Sequence");
impl Problem for Problem0014 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        (1..1_000_000)
            .map(|n| (n, collatz_seq(n).count()))
            .max_by_key(|&(_, count)| count)
            .unwrap()
            .0
            .to_string()
    }
}
