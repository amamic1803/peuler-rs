use crate::Problem;
use crate::math::prime::{apcf, sieve_of_eratosthenes};

problem!(Problem0007, 7, "10001st Prime");
impl Problem for Problem0007 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        sieve_of_eratosthenes(apcf(10001u64).round() as u64)[10_000].to_string()
    }
}
