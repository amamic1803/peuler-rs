use crate::Solution;
use pmath::digits::digits;

problem!(Problem0040, 40, "Champernowne's Constant");

impl Solution for Problem0040 {
    fn solve(&self) -> String {
        POSITIONS
            .into_iter()
            .map(|pos| get_digit(pos) as i32)
            .product::<i32>()
            .to_string()
    }
}

const POSITIONS: [u64; 7] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];

/// This function calculates the digit at the given position in Champernowne's Constant.
/// # Arguments
/// * `pos` - The position of the digit to calculate.
/// # Returns
/// * The digit at the given position.
fn get_digit(pos: u64) -> u8 {
    // there are 9 single digit numbers, 90 two digit numbers, 900 three digit numbers, etc.
    // 9 * n * 10^(n-1) is the number of digits in all numbers with n digits
    // iterate and sum until the sum is greater than the position
    // then take the previous sum as the starting point and calculate the number
    let mut n = 0;
    let mut prev_count = 0;
    let mut next_count = 0;
    while next_count <= pos {
        n += 1;
        prev_count = next_count;
        next_count += 9 * n * 10u64.pow(n as u32 - 1);
    }

    // calculate the last number with digits of length n - 1;
    let mut number = 10u64.pow(n as u32 - 1) - 1;

    // calculate how many digits still need to be moved
    let mut digits_to_move = pos - prev_count - 1;

    // since the numbers of length n are concerned, we can calculate how many full numbers to move
    // add that to number
    number += digits_to_move / n;

    // store how many digits to move in the next number
    digits_to_move %= n;

    // move to that next number (it contains the digit we are looking for)
    number += 1;

    // take the digit and return it, it takes from back since digits iterates from least to most significant
    // if the digits_to_move is zero, that means that we are at the digit we need to return, so that is index 0
    // we are also sure that the index won't be longer than the number of digits in the number
    // because that was checked before
    digits(number, 10)
        .nth_back(digits_to_move as usize)
        .unwrap()
}

// this is a little slower, but more straightforward solution that just stores all the digits in a vector
// fn solve() -> String {
//     let mut decimal_part: Vec<u64> = vec![];
//     let mut curr_num: u64 = 0;
//     while decimal_part.len() < 1_000_000 {
//         curr_num += 1;
//         let mut temp_num: u64 = curr_num;
//         let index = decimal_part.len();
//         while temp_num != 0 {
//             decimal_part.insert(index, temp_num % 10);
//             temp_num /= 10;
//         }
//     }
//     format!("{}", (decimal_part[1 - 1] * decimal_part[10 - 1] * decimal_part[100 - 1] * decimal_part[1_000 - 1] * decimal_part[10_000 - 1] * decimal_part[100_000 - 1] * decimal_part[1_000_000 - 1]))
// }
