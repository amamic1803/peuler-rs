use std::fs;

fn main() {
    let mut triangle_words: i64 = 0;
    let alphabet: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

    let words_input = fs::read_to_string("p042_words.txt").expect("Error while reading the file!");
    for word in words_input.split(",") {
        let mut word_value: i64 = 0;
        for letter in word.trim_start_matches('"').trim_end_matches('"').chars() {
            word_value += 1 + (alphabet.iter().position(|&x| x == letter).unwrap() as i64);
        }
        if is_triangle_number(word_value) {
            triangle_words += 1;
        }
    }
    println!("{}", triangle_words);
}

fn is_triangle_number(mut num: i64) -> bool {
    num *= 2;
    let square_root = (num as f64).sqrt() as i64;
    return if (square_root * (square_root + 1)) == num {
        true
    } else {
        false
    }
}