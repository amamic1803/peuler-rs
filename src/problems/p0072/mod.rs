use crate::Problem;
use crate::math::phi_0_to_n;

problem!(Problem0072, 72, "Counting Fractions");

impl Problem for Problem0072 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        // this is Farey sequence
        // the number of elements in the Farey sequence F(n) is given as:
        // F(n) = 1 + Σ(φ(i)) for i = 1 to n
        let farey_elements = 1 + phi_0_to_n(1000000).into_iter().sum::<u64>();

        // the problem excludes 0 and 1, so subtract 2 and return result
        (farey_elements - 2).to_string()
    }
}
