use crate::Problem;
use crate::math::primes::sieve_of_eratosthenes;

problem!(Problem0010, 10, "Summation of Primes");
impl Problem for Problem0010 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        const LIMIT: u64 = 2_000_000;
        sieve_of_eratosthenes(LIMIT - 1)
            .into_iter()
            .sum::<u64>()
            .to_string()
    }
}
