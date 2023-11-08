//! **Problem 29** - *Distinct Powers*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        29,
        "Distinct Powers",
        solve,
    )
}


fn solve() -> String {
    // let a and b be integers such that 2 <= a <= 100 and 2 <= b <= 100
    // let's define function f(a, b) = a^b
    // and function g(x) = log x
    // g(f(a, b)) = b * log a
    // since f is monotonic (increasing) for a,b >= 2, and g is monotonic (increasing) for x >= 0
    // f . g is also monotonic (increasing) for a,b >= 2
    // (composition of 2 strictly increasing functions is strictly increasing)
    // so f . g is injective for a,b >= 2
    // that means that we can find different values that f . g can take for a,b >= 2
    // the number of those values is the answer to the problem

    // the base for the logarithm doesn't matter as long as it is > 1

    let mut values = Vec::new();

    for a in 2..101 {
        let log_a = (a as f64).log2();
        for b in 2..101 {
            values.push(b as f64 * log_a);
        }
    }

    // now we need to determine how many of these values are distinct
    // since those are floating point numbers, we can't just test them for equality
    // because they have some error
    // we start by sorting the values
    // for every element we will check if the element before it is within 1e-7 of it
    // if it is, it is not distinct and we don't count it

    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut distinct = 1;
    for i in 1..values.len() {
        if (values[i] - values[i - 1]).abs() > 1e-7 {
            distinct += 1;
        }
    }

    distinct.to_string()
}


// this is slow, manual solution that actually calculates the powers
// fn solve() {
//     let mut powers: Vec<Vec<u64>> = vec![];
//
//     for a in 2..101 {
//         for b in 2..101 {
//             powers.push(power(a, b));
//         }
//     }
//
//     powers.sort();
//     let mut distinct: u64 = 1;
//
//     for i in 1..powers.len() {
//         if powers[i] != powers[i - 1] {
//             distinct += 1;
//         }
//     }
//
//     println!("{}", distinct);
// }
//
// fn power(base: u64, exp: u64) -> Vec<u64>{
//     let mut working_num: Vec<u64> = vec![1];
//
//     for _ in 0..exp {
//         let mut factoring: Vec<Vec<u64>> = vec![];
//         let mut factoring_num: u64 = base;
//
//         while factoring_num != 0 {
//             let mut current_product: Vec<u64> = vec![];
//
//             let mut transfer: u64 = 0;
//             for j in (0..working_num.len()).rev() {
//                 let working_product: u64 = (working_num[j] * (factoring_num % 10)) + transfer;
//                 current_product.insert(0, working_product % 10);
//                 transfer = working_product / 10;
//             }
//             while transfer != 0 {
//                 current_product.insert(0, transfer % 10);
//                 transfer /= 10;
//             }
//
//             factoring.push(current_product);
//
//             factoring_num /= 10;
//         }
//
//         for i in 0..factoring.len() {
//             for _  in 0..i {
//                 factoring[i].push(0)
//             }
//         }
//
//         let mut longest: usize = 0;
//
//         for i in 0..factoring.len() {
//             if factoring[i].len() > longest {
//                 longest = factoring[i].len();
//             }
//         }
//
//         for i in 0..factoring.len() {
//             for _ in 0..(longest - factoring[i].len()) {
//                 factoring[i].insert(0, 0);
//             }
//         }
//
//         let mut result: Vec<u64> = vec![];
//         let mut transfer: u64 = 0;
//         for j in (0..longest).rev() {
//             let mut working_sum: u64 = transfer;
//             for i in 0..factoring.len() {
//                 working_sum += factoring[i][j];
//             }
//             result.insert(0, working_sum % 10);
//             transfer = working_sum / 10;
//         }
//         while transfer != 0 {
//             result.insert(0, transfer % 10);
//             transfer /= 10;
//         }
//
//         working_num = result;
//
//     }
//     working_num
// }
