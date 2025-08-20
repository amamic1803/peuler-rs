use crate::Solution;

problem!(Problem0097, 97, "Large Non-Mersenne Prime");

impl Solution for Problem0097 {
    fn solve(&self) -> String {
        let mut result = MULTIPLIER;
        for _ in 0..EXPONENT {
            result = (result << 1) % MODULO; // using bitshift to multiply by 2 faster
        }
        result = (result + 1) % MODULO;
        result.to_string()
    }
}

// we take this modulo of the number to avoid overflow
// since we are only interested in the last 10 digits of the number, and we are only multiplying and adding, it is valid to take the modulo without affecting the result
const MODULO: u64 = 10_000_000_000;
const MULTIPLIER: u64 = 28433;
const EXPONENT: u64 = 7830457;
