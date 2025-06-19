//! **Problem 25** - *1000-digit Fibonacci Number*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(25, "1000-digit Fibonacci Number", solve)
}

fn solve() -> String {
    // let a = 1 + sqrt(5) / 2
    // let b = 1 - sqrt(5) / 2
    // we can use binet's formula to calculate the nth fibonacci number
    // f(n) = (a^n - b^n) / sqrt(5)

    // we know that wanted number needs to have at least 1000 digits
    // we first find the smallest number with 1000 digits, that is 10^999
    // now we want to find the first n for which f(n) >= 10^999

    // since b is smaller than 1, b^n will be very small for large n so we can ignore it
    // now binet's formula becomes f(n) = a^n / sqrt(5)

    // plugging that in the inequality we get a^n / sqrt(5) >= 10^999
    // after some manipulation, we get n >= (999 + log(5) / 2) / log(a)
    // now the only thing left is to find the smallest integer n that satisfies this inequality

    (((999.0_f64 + (5.0_f64.log10() / 2.0_f64)) / ((1.0_f64 + 5.0_f64.sqrt()) / 2.0_f64).log10())
        .ceil() as u64)
        .to_string()
}

// this is the slow, manual solution that actually calculates the fibonacci numbers
// fn solve() {
//     let mut fibonacci: Vec<Vec<u64>> = vec![vec![1], vec![1]];
//     let mut fibonacci_index: u64 = 2;
//
//     while fibonacci[1].len() < 1000 {
//         let mut result: Vec<u64> = vec![];
//
//         let mut transfer: u64 = 0;
//         for j in (0..fibonacci[1].len()).rev() {
//             let mut working_sum: u64 = transfer;
//             for i in 0..2 {
//                 working_sum += fibonacci[i][j];
//             }
//             result.insert(0, working_sum % 10);
//             transfer = working_sum / 10;
//         }
//         while transfer != 0 {
//             result.insert(0, transfer % 10);
//             transfer /= 10;
//         }
//
//         fibonacci.push(result);
//         fibonacci.remove(0);
//
//         for _ in 0..(fibonacci[1].len() - fibonacci[0].len()) {
//             fibonacci[0].insert(0, 0);
//         }
//
//         fibonacci_index += 1;
//     }
//
//     println!("{}", fibonacci_index);
// }
