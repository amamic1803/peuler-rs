use crate::Problem;
use crate::math::lcm_multiple;

problem!(Problem0005, 5, "Smallest Multiple");
impl Problem for Problem0005 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        lcm_multiple(1u32..=20).to_string()
    }
}
