use crate::Solution;
use pmath::digits::is_palindrome;

problem!(Problem0036, 36, "Double-base Palindromes");

impl Solution for Problem0036 {
    fn solve(&self) -> String {
        // the binary palindromes will be generated and checked if they are decimal palindromes
        // let xyz be a binary number with digits x, y, z
        // it is possible to generate two palindromes from xyz:
        // 1. xyzzyx
        // 2. xyzyx

        let mut sum = 0;

        // check for even length palindromes (xyzzyx)
        let mut x = 1;
        let mut generated_palindrome = binary_palindrome(x, true);
        while generated_palindrome < LIMIT {
            if is_palindrome(generated_palindrome, 10) {
                sum += generated_palindrome;
            }
            x += 1;
            generated_palindrome = binary_palindrome(x, true);
        }

        // check for odd length palindromes (xyzyx)
        x = 1;
        generated_palindrome = binary_palindrome(x, false);
        while generated_palindrome < LIMIT {
            if is_palindrome(generated_palindrome, 10) {
                sum += generated_palindrome;
            }
            x += 1;
            generated_palindrome = binary_palindrome(x, false);
        }

        sum.to_string()
    }
}

const LIMIT: u64 = 1_000_000;

/// Generate a number that is a palindrome in binary from a given number.
/// Assume that binary digits of the given number are `xyz`.
/// If `longer` is `true`, the generated number will be `xyzzyx`.
/// If `longer` is `false`, the generated number will be `xyzyx`.
fn binary_palindrome(mut x: u64, longer: bool) -> u64 {
    let mut result = x;
    if !longer {
        x >>= 1;
    }
    while x != 0 {
        result <<= 1;
        result |= x & 1;
        x >>= 1;
    }
    result
}
