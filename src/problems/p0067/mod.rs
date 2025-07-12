use crate::Problem;
use std::cmp::max;

problem!(Problem0067, 67, "Maximum Path Sum II");

impl Problem for Problem0067 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let input_triangle_str = include_str!("0067_triangle.txt");
        max_path_sum_triangle(input_triangle_str).to_string()
    }
}

pub(crate) fn max_path_sum_triangle(input: &str) -> u64 {
    let mut input_triangle = input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    // start from the second to last row and for every number
    // add to it the bigger number in the row under it

    for i in (0..(input_triangle.len() - 1)).rev() {
        for j in 0..input_triangle[i].len() {
            input_triangle[i][j] += max(input_triangle[i + 1][j], input_triangle[i + 1][j + 1]);
        }
    }

    input_triangle[0][0]
}
