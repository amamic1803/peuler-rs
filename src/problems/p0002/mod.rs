use crate::Problem;
use crate::math::sequence::FibonacciSeq;

problem!(Problem0002, 2, "Even Fibonacci Numbers");
impl Problem for Problem0002 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
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
