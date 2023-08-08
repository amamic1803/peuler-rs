use std::cmp::max;
use std::fs;

fn main() {
    // logic - go from the line before the last one and for every number add the bigger in the line under it.
    let input = fs::read_to_string("p067_triangle.txt").expect("Error while reading the file!");

    let mut input_vec: Vec<Vec<u64>> = vec![];

    for i in input.split("\n") {
        let mut line: Vec<u64> = vec![];
        for j in i.split(" ") {
            if j != "" {
                line.push(j.parse().unwrap());
            }
        }
        if line.len() != 0 {
            input_vec.push(line);
        }
    }

    for row in (0..(input_vec.len() - 1)).rev() {
        for element in 0..input_vec[row].len() {
            input_vec[row][element] += max(input_vec[row + 1][element], input_vec[row + 1][element + 1]);
        }
    }

    println!("{}", input_vec[0][0]);
}
