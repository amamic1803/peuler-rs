use std::fs;
use std::collections::HashMap;

fn main() {
    let mut alphabet_hash = HashMap::new();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut index: u64 = 1;
    for letter in alphabet.chars() {
        alphabet_hash.insert(letter,  index);
        index += 1;
    }

    let names_input = fs::read_to_string("p022_names.txt").expect("Error while reading the file!");

    let mut names_vec: Vec<&str> = vec![];

    for name in names_input.split(",") {
        names_vec.push(name.trim_start_matches('"').trim_end_matches('"'));
    }

    names_vec.sort();

    let mut result: u64 = 0;
    for i in 0..names_vec.len() {
        let mut sum: u64 = 0;
        for j in names_vec[i].chars() {
            sum += alphabet_hash[&j];
        }
        result += sum * ((i as u64) + 1);
    }

    println!("{}", result);
}
