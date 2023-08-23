//! **Problem 18** - *Maximum Path Sum I*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        18,
        "Maximum Path Sum I",
        solve,
    )
}


use std::cmp::max;

fn solve() -> String {
    let input_triangle_str = "75\n95 64\n17 47 82\n18 35 87 10\n20 04 82 47 65\n19 01 23 75 03 34\n88 02 77 73 07 63 67\n99 65 04 28 06 16 70 92\n41 41 26 56 83 40 80 70 33\n41 48 72 33 47 32 37 16 94 29\n53 71 44 65 25 43 91 52 97 51 14\n70 11 33 28 77 73 17 78 39 68 17 57\n91 71 52 38 17 14 91 43 58 50 27 29 48\n63 66 04 68 89 53 67 30 73 16 69 87 40 31\n04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";
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
        .map(|line| line
            .split_whitespace()
            .map(|num_str| num_str
                .parse::<u64>().unwrap()
            ).collect::<Vec<u64>>()
        ).collect::<Vec<Vec<u64>>>()
}
