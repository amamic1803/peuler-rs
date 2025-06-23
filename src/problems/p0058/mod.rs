use crate::Problem;
use crate::math::prime::is_prime;

problem!(Problem0058, 58, "Spiral Primes");

impl Problem for Problem0058 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let mut primes: u32 = 3;
        let mut nums: u32 = 5;
        let mut size: u32 = 2;

        let mut top_left: [u64; 2] = [5, 4];
        let mut top_right: [u64; 2] = [3, 2];
        let mut bottom_left: [u64; 2] = [7, 6];
        let mut bottom_right: [u64; 2] = [9, 8];

        while (primes as f64) / (nums as f64) >= 0.1 {
            size += 1;
            nums += 4;

            top_left[1] += 8;
            top_right[1] += 8;
            bottom_left[1] += 8;
            bottom_right[1] += 8;

            top_left[0] += top_left[1];
            top_right[0] += top_right[1];
            bottom_left[0] += bottom_left[1];
            bottom_right[0] += bottom_right[1];

            if is_prime(top_left[0]).0 {
                primes += 1;
            }
            if is_prime(top_right[0]).0 {
                primes += 1;
            }
            if is_prime(bottom_left[0]).0 {
                primes += 1;
            }
            if is_prime(bottom_right[0]).0 {
                primes += 1;
            }
        }

        (2 * size - 1).to_string()
    }
}
