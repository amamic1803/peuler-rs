//! **Problem 17** - *Number Letter Counts*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        17,
        "Number Letter Counts",
        solve,
    )
}


use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;


fn solve() -> String {
    let mut sum = 0;
    for n in 1..1001 {
        sum += num_to_string(n).chars().filter(|c| ![' ', '-'].contains(c)).count();
    }
    sum.to_string()
}

fn num_to_string(mut n: u64) -> String {
    //! Returns the plain english name of a number.
    //! (smaller than or equal to 1000)

    let mut name = String::new();

    if n == 1000 {
        name.push_str("one thousand");
    } else {
        let hundreds: u64 =  n / 100;
        if hundreds != 0 {
            name.push_str(DIGIT_SINGLE.lock().unwrap().get(&hundreds).unwrap());
            name.push_str(" hundred")
        }
        n %= 100;

        if n != 0 {
            if hundreds != 0 {
                name.push_str(" and ");
            }
            if (9 < n) & (n < 20) {
                name.push_str(DIGIT_TEEN.lock().unwrap().get(&n).unwrap());
            } else if n < 10 {
                name.push_str(DIGIT_SINGLE.lock().unwrap().get(&n).unwrap());
            } else {
                let tens: u64 = n / 10;
                let ones: u64 = n % 10;
                name.push_str(DIGIT_DOUBLE.lock().unwrap().get(&tens).unwrap());
                if ones != 0 {
                    name.push('-');
                    name.push_str(DIGIT_SINGLE.lock().unwrap().get(&ones).unwrap());
                }
            }
        }
    }

    name
}

static DIGIT_SINGLE: Lazy<Mutex<HashMap<u64, &str>>> = Lazy::new(|| {
    let mut hash_map = HashMap::new();

    hash_map.insert(1, "one");
    hash_map.insert(2, "two");
    hash_map.insert(3, "three");
    hash_map.insert(4, "four");
    hash_map.insert(5, "five");
    hash_map.insert(6, "six");
    hash_map.insert(7, "seven");
    hash_map.insert(8, "eight");
    hash_map.insert(9, "nine");

    Mutex::new(hash_map)
});

static DIGIT_DOUBLE: Lazy<Mutex<HashMap<u64, &str>>> = Lazy::new(|| {
    let mut hash_map = HashMap::new();

    hash_map.insert(2, "twenty");
    hash_map.insert(3, "thirty");
    hash_map.insert(4, "forty");
    hash_map.insert(5, "fifty");
    hash_map.insert(6, "sixty");
    hash_map.insert(7, "seventy");
    hash_map.insert(8, "eighty");
    hash_map.insert(9, "ninety");

    Mutex::new(hash_map)
});

static DIGIT_TEEN: Lazy<Mutex<HashMap<u64, &str>>> = Lazy::new(|| {
    let mut hash_map = HashMap::new();

    hash_map.insert(10, "ten");
    hash_map.insert(11, "eleven");
    hash_map.insert(12, "twelve");
    hash_map.insert(13, "thirteen");
    hash_map.insert(14, "fourteen");
    hash_map.insert(15, "fifteen");
    hash_map.insert(16, "sixteen");
    hash_map.insert(17, "seventeen");
    hash_map.insert(18, "eighteen");
    hash_map.insert(19, "nineteen");

    Mutex::new(hash_map)
});