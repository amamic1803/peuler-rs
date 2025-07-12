use crate::Problem;
use crate::math::digits::digits_to_int;
use crate::math::primes::is_prime;
use itertools::Itertools;

problem!(Problem0041, 41, "Pandigital Prime");

impl Problem for Problem0041 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        // since a 4 digit pandigital prime is given in the problem,
        // there is no need to check numbers with less than 4 digits
        // now let's consider divisibility by 3:
        // 4 digit pandigital numbers are 1+2+3+4 = 10, which is not divisible by 3
        // 5 digit pandigital numbers are 1+2+3+4+5 = 15, which is divisible by 3
        // 6 digit pandigital numbers are 1+2+3+4+5+6 = 21, which is divisible by 3
        // 7 digit pandigital numbers are 1+2+3+4+5+6+7 = 28, which is not divisible by 3
        // 8 digit pandigital numbers are 1+2+3+4+5+6+7+8 = 36, which is divisible by 3
        // 9 digit pandigital numbers are 1+2+3+4+5+6+7+8+9 = 45, which is divisible by 3
        // so pandigital prime can only be 4 or 7 digits long
        // now we check all those numbers and return the largest one

        let mut largest_prime = 0;

        for len in [4, 7] {
            for num in (1..=len).permutations(len as usize) {
                let num = digits_to_int(num, 10);
                if is_prime(num).0 && num > largest_prime {
                    largest_prime = num;
                }
            }
        }

        largest_prime.to_string()
    }
}
