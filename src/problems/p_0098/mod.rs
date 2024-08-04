//! **Problem 98** - *Anagramic Squares*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(98, "Anagramic Squares", solve)
}

use crate::shared::math::{digits, is_permutation};
use itertools::Itertools;

fn solve() -> String {
    // input string of words
    let input_str = include_str!("0098_words.txt");

    // split input string into vector of tuples of (word, sorted characters of word)
    let mut words_vec: Vec<(&str, Vec<char>)> = input_str
        .split(',')
        .map(|word| {
            let word = word.trim_matches('"');
            let mut word_chars: Vec<char> = word.chars().collect();
            word_chars.sort();
            (word, word_chars)
        })
        .collect();

    // we first want to find pairs of words that are anagrams
    let mut anagram_pairs = Vec::new();

    // keep popping words from the vector of words
    while let Some((word_str, word_chars)) = words_vec.pop() {
        // for each word define a group of anagrams (all words with the same sorted characters)
        let mut anagram_group = vec![word_str];

        // loop through the vector of words and find all anagrams of the current word
        // add them to the anagram group and remove them from the vector of words
        let mut i = 0;
        while i < words_vec.len() {
            if word_chars == words_vec[i].1 {
                anagram_group.push(words_vec[i].0);
                words_vec.remove(i);
            } else {
                i += 1;
            }
        }

        // if the anagram group has more than 1 word,
        // add all combinations of 2 words from the group to the anagram pairs vector
        for combination in anagram_group.into_iter().combinations(2) {
            anagram_pairs.push((combination[0], combination[1]));
        }
    }

    // now we have a vector with pairs of words from the input that are anagrams
    // we can now process each of these pairs in the way described in the problem
    // to find the solution

    // sort the pairs by the length of the words
    // this is done because we want to process from longer words
    // because we want the maximum square number
    anagram_pairs.sort_by_key(|pair| pair.0.len());

    // assert that the maximum length of the words is <= 10
    // (so that if all letters are unique, they can uniquely be represented by digits 0-9)
    assert!(anagram_pairs.last().unwrap().0.len() <= 10);

    // the maximum square number found (the solution)
    let mut maximum: u64 = 0;

    // process each pair of anagram words (from the longest, at the back, to the shortest at the front)
    while let Some(word_pair) = anagram_pairs.pop() {
        // if the maximum square number found is > 0,
        // and it's length is greater than the length of the current word from pair,
        // we can break the loop since we are looping from longer to shorter words
        if maximum != 0 && word_pair.0.len() < maximum.ilog10() as usize + 1 {
            break;
        }

        // extract the words from the pair
        let (word1, word2) = (word_pair.0, word_pair.1);

        // for each word we will find all square numbers that can represent it
        // and for each of those numbers store the array
        // that links the digits of the number to the letters of the word
        // (position of the array is the digit, value is the letter)
        let (mut storage1, mut storage2) = (Vec::new(), Vec::new());

        // iterator over squares with the same number of digits as the words
        let squares_iter =
            (((10_u64.pow(word1.len() as u32 - 1) as f64).sqrt().ceil() as u64)..=((10_u64.pow(word1.len() as u32) as f64).sqrt().floor() as u64)).map(|i| i * i);

        // for each square number, find if it can represent each word
        // if it can, push it and the array to the storage vector of that word
        for square in squares_iter {
            let match_1 = word_matches_num(word1, square);
            if match_1.0 {
                storage1.push((square, match_1.1));
            }
            let match_2 = word_matches_num(word2, square);
            if match_2.0 {
                storage2.push((square, match_2.1));
            }
        }

        // now we need to connect the square numbers that can represent both words
        // those need to be permutations of each other
        // and the arrays that link the digits to the letters need to be the same
        // (because same letters need to be represented by the same digits)
        // if we find such a pair, we update the maximum square number found
        for (sqr1, arr1) in storage1 {
            for (sqr2, arr2) in &storage2 {
                if is_permutation(sqr1, *sqr2, 10) && arr1 == *arr2 {
                    maximum = maximum.max(sqr1).max(*sqr2);
                }
            }
        }
    }

    // return the maximum square number found
    maximum.to_string()
}

/// Check whether the word can be represented by the number
/// Returns a tuple of a boolean (whether the word can be represented by the number)
/// and an array that links the digits of the number to the letters of the word.
/// Array is meaningless if the boolean is false.
/// Array is of length 10, where the position of the array is the digit, and the value is the letter.
fn word_matches_num(word: &str, num: u64) -> (bool, [char; 10]) {
    // array that links the digits of the number to the letters of the word
    let mut num_char_dict = [' '; 10];

    // match characters of the word to digits of the number
    // if a digit is already matched to a character, and the current character is different,
    // but should be matched to the same digit, return false
    for (n, c) in digits(num, 10).zip(word.chars()) {
        if num_char_dict[n as usize] == ' ' {
            num_char_dict[n as usize] = c;
        } else if num_char_dict[n as usize] != c {
            return (false, num_char_dict);
        }
    }

    // check whether the same character is matched to multiple digits (not allowed by the problem)
    for c in num_char_dict.iter() {
        if *c != ' ' && num_char_dict.iter().filter(|&x| *x == *c).count() > 1 {
            return (false, num_char_dict);
        }
    }

    // if all checks passed, return true and the array
    (true, num_char_dict)
}
