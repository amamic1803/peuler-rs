use crate::Problem;

problem!(Problem0009, 9, "Special Pythagorean Triplet");
impl Problem for Problem0009 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let limit = 1000;

        for a in 1..(limit / 3 + 1) {
            for b in a..((limit - a) / 2 + 1) {
                let c = limit - a - b;
                if a * a + b * b == c * c {
                    return (a * b * c).to_string();
                }
            }
        }

        "No solution found!".to_string()
    }
}
