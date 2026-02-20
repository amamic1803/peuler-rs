use crate::Solution;
use pmath::digits::digits_to_int;
use std::collections::HashMap;
use std::sync::LazyLock;

problem!(Problem0043, 43, "Sub-string Divisibility");

impl Solution for Problem0043 {
    fn solve(&self) -> String {
        let mut sum = 0;
        let mut working = Vec::new();

        recursive_search(&mut working, &mut sum, 1);

        sum.to_string()
    }
}

/// The digits that can be used in each position.
static DIGITS: LazyLock<HashMap<u64, Vec<u64>>> = LazyLock::new(|| {
    let mut digits = HashMap::with_capacity(10);
    digits.insert(1, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    digits.insert(2, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    digits.insert(3, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    digits.insert(4, vec![0, 2, 4, 6, 8]);
    digits.insert(5, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    digits.insert(6, vec![0, 5]);
    digits.insert(7, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    digits.insert(8, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    digits.insert(9, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    digits.insert(10, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    digits
});

/// Recursively search for all pandigital numbers with the given properties.
fn recursive_search(working: &mut Vec<u64>, sum: &mut u64, depth: u64) {
    // if depth is 1, just use every digit for that position
    // (there is no need to check if the digit is already used since it is the first digit)
    // first property needs to be checked at d4, so for 2 and 3
    // just use every digit (but check if the digit was already used)
    // also for d4 use every digit since divisibility by 2 is ensured by using even digits for d4
    // the same goes for d6 (divisibility by 5 is ensured by using 0 or 5 for d6)
    // for other positions, use every digit in the position, checking the divisibility property
    // and if the divisibility property is satisfied, continue the recursive search
    // if we get to d11, we have a pandigital number with the given properties

    match depth {
        1 => {
            for i in DIGITS.get(&depth).unwrap() {
                working.push(*i);
                recursive_search(working, sum, depth + 1);
                working.pop();
            }
        }
        2..=4 | 6 => {
            for i in DIGITS.get(&depth).unwrap() {
                if !working.contains(i) {
                    working.push(*i);
                    recursive_search(working, sum, depth + 1);
                    working.pop();
                }
            }
        }
        5 => {
            for i in DIGITS.get(&depth).unwrap() {
                if !working.contains(i) {
                    working.push(*i);
                    let value: u64 = digits_to_int(working[2..5].iter().rev(), 10);
                    if value.is_multiple_of(3) {
                        recursive_search(working, sum, depth + 1);
                    }
                    working.pop();
                }
            }
        }
        7 => {
            for i in DIGITS.get(&depth).unwrap() {
                if !working.contains(i) {
                    working.push(*i);
                    let value: u64 = digits_to_int(working[4..7].iter().rev(), 10);
                    if value.is_multiple_of(7) {
                        recursive_search(working, sum, depth + 1);
                    }
                    working.pop();
                }
            }
        }
        8 => {
            for i in DIGITS.get(&depth).unwrap() {
                if !working.contains(i) {
                    working.push(*i);
                    let value: u64 = digits_to_int(working[5..8].iter().rev(), 10);
                    if value.is_multiple_of(11) {
                        recursive_search(working, sum, depth + 1);
                    }
                    working.pop();
                }
            }
        }
        9 => {
            for i in DIGITS.get(&depth).unwrap() {
                if !working.contains(i) {
                    working.push(*i);
                    let value: u64 = digits_to_int(working[6..9].iter().rev(), 10);
                    if value.is_multiple_of(13) {
                        recursive_search(working, sum, depth + 1);
                    }
                    working.pop();
                }
            }
        }
        10 => {
            for i in DIGITS.get(&depth).unwrap() {
                if !working.contains(i) {
                    working.push(*i);
                    let value: u64 = digits_to_int(working[7..10].iter().rev(), 10);
                    if value.is_multiple_of(17) {
                        recursive_search(working, sum, depth + 1);
                    }
                    working.pop();
                }
            }
        }
        11 => {
            let value: u64 = digits_to_int(working.iter().rev(), 10);
            *sum += value;
        }
        _ => unreachable!("Invalid depth"),
    }
}
