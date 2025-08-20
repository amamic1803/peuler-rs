use crate::Solution;
use std::cmp::min;

problem!(Problem0081, 81, "Path Sum: Two Ways");

impl Solution for Problem0081 {
    fn solve(&self) -> String {
        const INPUT: &str = include_str!("p081_matrix.txt");
        let mut matrix = INPUT
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .split(',')
                    .map(|num_str| num_str.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        // we start from the bottom right corner and work our way up
        // to each position we add the smaller between the right and bottom values

        // first let's handle special cases

        // last/bottom row
        let last_row = matrix.len() - 1;
        for i in (0..(matrix[last_row].len() - 1)).rev() {
            matrix[last_row][i] += matrix[last_row][i + 1];
        }

        // last/right column
        let last_col = matrix[0].len() - 1;
        for i in (0..(matrix.len() - 1)).rev() {
            matrix[i][last_col] += matrix[i + 1][last_col];
        }

        // finally, we do this for general cases
        for i in (0..(matrix.len() - 1)).rev() {
            for j in (0..(matrix[0].len() - 1)).rev() {
                matrix[i][j] += min(matrix[i][j + 1], matrix[i + 1][j]);
            }
        }

        // the first cell now contains the minimal path sum
        matrix[0][0].to_string()
    }
}
