use crate::Problem;
use crate::math::is_prime;

problem!(Problem0003, 3, "Largest Prime Factor");
impl Problem for Problem0003 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        // every iteration check if the number is prime
        // if it is, then it is the largest prime factor
        // if it is not, then divide the number by the smallest divisor (returned by is_prime().1)

        let mut given_num: u64 = 600851475143;
        loop {
            let result = is_prime(given_num);
            if result.0 {
                break;
            } else {
                given_num /= result.1;
            }
        }
        given_num.to_string()
    }
}
