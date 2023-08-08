use std::cmp::min;
use std::fs;

fn main() {
    // logic - go backwards from bottom right and add lower value (right or down) to every number
    let input = fs::read_to_string("p081_matrix.txt").expect("Error while reading the file!");

    let mut input_vec: Vec<Vec<u64>> = vec![];
    for i in input.split("\n") {
        let mut line: Vec<u64> = vec![];
        for j in i.split(",") {
            if j != "" {
                line.push(j.parse().unwrap());
            }
        }
        if line.len() != 0 {
            input_vec.push(line);
        }
    }

    for i in (0..input_vec.len()).rev() {
        for j in (0..input_vec[0].len()).rev() {
            if (i == input_vec.len() - 1) & (j == input_vec[0].len() - 1) {
                continue;
            } else if i == input_vec.len() - 1 {
                input_vec[i][j] += input_vec[i][j + 1];
            } else if j == input_vec[0].len() - 1 {
                input_vec[i][j] += input_vec[i + 1][j];
            } else {
                input_vec[i][j] += min(input_vec[i][j + 1], input_vec[i + 1][j]);
            }
        }
    }

    println!("{}", input_vec[0][0]);
}
