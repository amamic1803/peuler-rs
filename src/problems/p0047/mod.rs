use crate::Problem;
use crate::math::prime::distinct_prime_factors;

problem!(Problem0047, 47, "Distinct Primes Factors");

impl Problem for Problem0047 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        // number of consecutive numbers to find
        const CONSECUTIVE_COUNT: u64 = 4;

        // array to store numbers with distinct prime factors
        // we search until this vector contains exclusively consecutive numbers
        // and then we return the first number in the vector (solution)
        let mut consecutive = [0; CONSECUTIVE_COUNT as usize];

        // we start search with the first number that has 4 distinct prime factors
        let mut curr_num = 2 * 3 * 5 * 7;

        // n + (n + 1) + (n + 2) + (n + 3) = 4n + 1 + 2 + 3
        // obviously if the array contains consecutive numbers, the sum of the array
        // must be equal to the (CONSECUTIVE_COUNT * n) + 1 + 2 + ... + (CONSECUTIVE_COUNT - 1)
        // which is equal to (CONSECUTIVE_COUNT * n) + (((CONSECUTIVE_COUNT - 1) * CONSECUTIVE_COUNT) / 2)
        // so we search until the sum of the array is not equal to the above expression

        while consecutive.iter().sum::<u64>()
            != (consecutive[0] * CONSECUTIVE_COUNT
                + (((CONSECUTIVE_COUNT - 1) * CONSECUTIVE_COUNT) / 2))
        {
            // if current number has 4 distinct prime factors, add it to the end of the array and remove the first number
            // this is actually achieved by setting the first element to current number and rotating the array left by 1
            if distinct_prime_factors(curr_num).count() == CONSECUTIVE_COUNT as usize {
                consecutive[0] = curr_num;
                consecutive.rotate_left(1);
            }

            // increment current number
            curr_num += 1;
        }

        // return the first number in the array
        consecutive[0].to_string()
    }
}
