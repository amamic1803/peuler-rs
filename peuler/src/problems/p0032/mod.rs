use crate::Solution;
use itertools::Itertools;
use pmath::digits::digits_to_int;
use std::collections::HashSet;

problem!(Problem0032, 32, "Pandigital Products");

impl Solution for Problem0032 {
    fn solve(&self) -> String {
        // if we list combinations of multiplicand and multiplier digits:
        // 1 digit * 3 digits = 4 digits            (total 8 digits)
        // 1 digit * 4 digits = 4 or 5 digits       (total 9 or 10 digits)
        // 1 digit * 5 digits = 5 digits            (total 11 digits)
        // 2 digits * 2 digits = 3 or 4 digits      (total 7 or 8 digits)
        // 2 digits * 3 digits = 4 or 5 digits      (total 9 or 10 digits)
        // 2 digits * 4 digits = 5 digits           (total 11 digits)
        // only combinations that can result in 9 total digits are 1, 4 and 2, 3
        // naturally, 4, 1 and 3, 2 also result in 9 total digits, but they will result in the same products

        let mut sum = 0;
        let mut found_products = HashSet::new();

        for perm in (1..=9).permutations(9) {
            for first_digit_len in 1..=2 {
                let fact1: i32 = digits_to_int(&perm[..first_digit_len], 10);
                let fact2: i32 = digits_to_int(&perm[first_digit_len..5], 10);
                let prod: i32 = digits_to_int(&perm[5..], 10);
                if fact1 * fact2 == prod && !found_products.contains(&prod) {
                    sum += prod;
                    found_products.insert(prod);
                }
            }
        }

        sum.to_string()
    }
}
