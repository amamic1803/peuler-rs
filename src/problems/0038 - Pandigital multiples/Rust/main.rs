fn main() {
    solve_38();
}

fn solve_38() {
    let digits: Vec<u128> = (1..10).collect();
    let mut max_pandigital: u128 = 0;
    for number in 1..10_000 {
        let mut multiples: Vec<u128> = vec![];
        let mut curr_multiplier: u128 = 1;
        while len_of_numbers(&multiples) < 9 {
            multiples.push(number * curr_multiplier);
            curr_multiplier += 1;
        }
        if is_pandigital(&multiples, &digits) {
            let concatenated = concatenate_numbers(&multiples);
            if concatenated > max_pandigital {
                max_pandigital = concatenated;
            }
        }
    }
    println!("{}", max_pandigital);
}

fn is_pandigital(numbers: &Vec<u128>, digits: &Vec<u128>) -> bool {
    let mut found_digits: Vec<u128> = vec![];

    for index in 0..numbers.len() {
        let mut temp_n = numbers[index];
        while temp_n != 0 {
            found_digits.push(temp_n % 10);
            temp_n /= 10;
        }
    }

    found_digits.sort();

    return if digits == &found_digits {
        true
    } else {
        false
    }
}

fn len_of_numbers(numbers: &Vec<u128>) -> u128 {
    let mut result: u128 = 0;
    for position in 0..numbers.len() {
        let mut n: u128 = numbers[position];
        while n != 0 {
            result += 1;
            n /= 10;
        }
    }
    result
}

fn concatenate_numbers(numbers: &Vec<u128>) -> u128 {
    let mut concatenated_string = String::new();
    for number in numbers {
        concatenated_string.push_str(&number.to_string());
    }
    concatenated_string.parse().unwrap()
}