//! **Problem 67** - *Maximum Path Sum II*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(67, "Maximum Path Sum II", solve)
}

use std::cmp::max;

fn solve() -> String {
    let input_triangle_str = include_str!("0067_triangle.txt");
    let mut input_triangle = parse_input(input_triangle_str);

    // start from the second to last row and for every number add to it the bigger in the row under it

    for i in (0..(input_triangle.len() - 1)).rev() {
        for j in 0..input_triangle[i].len() {
            input_triangle[i][j] += max(input_triangle[i + 1][j], input_triangle[i + 1][j + 1]);
        }
    }

    input_triangle[0][0].to_string()
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().map(|num_str| num_str.parse::<u64>().unwrap()).collect::<Vec<u64>>())
        .collect::<Vec<Vec<u64>>>()
}
