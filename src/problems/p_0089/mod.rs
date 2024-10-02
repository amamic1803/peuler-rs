//! **Problem 89** - *Roman Numerals*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(89, "Roman Numerals", solve)
}

fn solve() -> String {
    let roman_numerals = include_str!("0089_roman.txt");
    let mut saved_chars = 0;

    for roman_numeral in roman_numerals.lines() {
        saved_chars += roman_numeral.chars().count();
        saved_chars -= int_2_roman(roman_2_int(roman_numeral)).chars().count();
    }

    saved_chars.to_string()
}

fn int_2_roman(mut num: u16) -> String {
    let mut roman = String::with_capacity(4);

    for _ in 0..num / 1000 {
        roman.push('M');
    }
    num %= 1000;

    let hundreds = num / 100;
    num %= 100;
    match hundreds {
        9 => roman.push_str("CM"),
        5..=8 => {
            roman.push('D');
            (5..hundreds).for_each(|_| roman.push('C'));
        }
        4 => roman.push_str("CD"),
        _ => (0..hundreds).for_each(|_| roman.push('C')),
    }

    let tens = num / 10;
    num %= 10;
    match tens {
        9 => roman.push_str("XC"),
        5..=8 => {
            roman.push('L');
            (5..tens).for_each(|_| roman.push('X'));
        }
        4 => roman.push_str("XL"),
        _ => (0..tens).for_each(|_| roman.push('X')),
    }

    match num {
        9 => roman.push_str("IX"),
        5..=8 => {
            roman.push('V');
            (5..num).for_each(|_| roman.push('I'));
        }
        4 => roman.push_str("IV"),
        _ => (0..num).for_each(|_| roman.push('I')),
    }

    roman
}

fn roman_2_int(roman: &str) -> u16 {
    let mut integer = 0;

    let mut iter = roman.chars().peekable();
    while let Some(c) = iter.next() {
        match c {
            'I' => match iter.peek() {
                Some('V') => {
                    integer += 4;
                    iter.next();
                }
                Some('X') => {
                    integer += 9;
                    iter.next();
                }
                _ => integer += 1,
            },
            'V' => integer += 5,
            'X' => match iter.peek() {
                Some('L') => {
                    integer += 40;
                    iter.next();
                }
                Some('C') => {
                    integer += 90;
                    iter.next();
                }
                _ => integer += 10,
            },
            'L' => integer += 50,
            'C' => match iter.peek() {
                Some('D') => {
                    integer += 400;
                    iter.next();
                }
                Some('M') => {
                    integer += 900;
                    iter.next();
                }
                _ => integer += 100,
            },
            'D' => integer += 500,
            'M' => integer += 1000,
            _ => (),
        }
    }

    integer
}
