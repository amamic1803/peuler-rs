use crate::Problem;
use crate::math::sieve_of_eratosthenes;

problem!(Problem0010, 10, "Summation of Primes");
impl Problem for Problem0010 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        sieve_of_eratosthenes(1_999_999)
            .into_iter()
            .sum::<u64>()
            .to_string()
    }
}
