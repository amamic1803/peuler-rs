fn main() {
    let mut sum: i64 = 0;
    let mut found_primes: u8 = 0;

    let first_digit: [i64; 4] = [2, 3, 5, 7];
    let middle_digit: [i64; 4] = [1, 3, 7, 9];
    let last_digit: [i64; 2] = [3, 7];

    let mut curr_len: i64 = 2;
    while found_primes < 11 {
        let mut possibilities: Vec<Vec<i64>> = vec![];

        // first digit + last digit
        for digit_1 in first_digit {
            for digit_2 in last_digit {
                possibilities.push(vec![digit_1, digit_2]);
            }
        }

        // middle digits
        for _ in 0..(curr_len - 2) {
            let mut added: usize = 0;
            for index in 0..possibilities.len() {
                let old_added: usize = added;
                for digit in middle_digit {
                    possibilities.insert(index + added + 1, possibilities[index + old_added].clone());
                    possibilities[index + added + 1].insert(1, digit);
                    added += 1;
                }
                possibilities.remove(index + old_added);
                added -= 1;
            }
        }

        // checking number meet criteria
        for x in possibilities {
            let integer = vec_to_int(x);
            if is_truncatable_prime(integer) {
                found_primes += 1;
                sum += integer;
                if found_primes == 11 {
                    break;
                }
            }
        }

        curr_len += 1;
    }
    println!("{}", sum);
}

fn is_truncatable_prime(x: i64) -> bool {
    if !is_prime(x) {
        return false;
    }
    for i in 1..(len_of_number(x) as u32) {
        if !is_prime(x % (10_i64.pow(i))) || !is_prime(x / (10_i64.pow(i))) {
            return false;
        }
    }
    true
}

fn is_prime(x: i64) -> bool {
    return if x < 2 {
        false
    } else if x == 2 {
        true
    } else if x % 2 == 0 {
        false
    } else {
        for i in (3..=((x as f64).sqrt() as i64)).step_by(2) {
            if x % i == 0 {
                return false
            }
        }
        true
    }
}

fn vec_to_int(vector: Vec<i64>) -> i64 {
    let mut integer: i64 = 0;
    for ind in 0..vector.len() {
        integer += vector[ind] * 10_i64.pow((vector.len() - 1 - ind) as u32);
    }
    integer
}

fn len_of_number(mut n: i64) -> i64 {
    let mut result: i64 = 0;
    while n != 0 {
        result += 1;
        n /= 10;
    }
    result
}