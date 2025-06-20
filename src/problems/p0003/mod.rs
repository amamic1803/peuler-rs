use crate::Problem;
use crate::math::primes::prime_factors;

problem!(Problem0003, 3, "Largest Prime Factor");
impl Problem for Problem0003 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        const TARGET: u64 = 600851475143;
        prime_factors(TARGET).max().unwrap().to_string()
    }
}
