//! **Problem 99** - *Largest Exponential*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(99, "Largest Exponential", solve)
}

fn solve() -> String {
    let input_str = include_str!("0099_base_exp.txt");
    let base_exp_pairs = parse_input(input_str);

    // the numbers we are working with are far too large to calculate
    // but we can take the logarithm of those numbers and compare the logarithms instead
    // that is valid because logarithm is strictly increasing function
    // that is if x2 > x1, then log(x2) > log(x1)
    // since logarithms are much smaller numbers, they are relatively easy to calculate
    // note that we don't care what base we use for the logarithm as long as it is bigger than 1
    // (since all logarithms with base > 1 are strictly increasing functions)

    let mut max_log = f64::NEG_INFINITY;
    let mut max_ind = 0;

    for (i, (base, exp)) in base_exp_pairs.into_iter().enumerate() {
        let logarithm = (exp as f64) * (base as f64).log2();
        if logarithm > max_log {
            max_log = logarithm;
            max_ind = i + 1;
        }
    }

    max_ind.to_string()
}

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut line_iter = line.trim().split(',').map(|num_str| num_str.parse::<u32>().unwrap());
            (line_iter.next().unwrap(), line_iter.next().unwrap())
        })
        .collect::<Vec<(u32, u32)>>()
}
