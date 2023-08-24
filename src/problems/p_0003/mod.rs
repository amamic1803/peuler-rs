//! **Problem 3** - *Largest Prime Factor*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        3,
        "Largest Prime Factor",
        solve,
    )
}


use crate::shared::math::is_prime;

fn solve() -> String {
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