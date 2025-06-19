use crate::Problem;

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
        let mut values = [1, 2];
        while values[1] < 4000000 {
            if values[1] % 2 == 0 {
                sum += values[1];
            }
            (values[0], values[1]) = (values[1], values[0] + values[1]);
        }
        sum.to_string()
    }
}
