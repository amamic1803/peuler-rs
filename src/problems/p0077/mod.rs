use crate::Problem;
use crate::math::partition_prime;

problem!(Problem0077, 77, "Prime Summations");

impl Problem for Problem0077 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        for i in 0u32.. {
            if partition_prime(i) > 5000 {
                return i.to_string();
            }
        }
        unreachable!()
    }
}
