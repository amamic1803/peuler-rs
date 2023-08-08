fn main() {
    let mut number: u64 = 1000000;
    loop {
        if if_palindrome(number) & if_3dig_fact(number) {
            break;
        } else {
            number -= 1;
        }

    }
    println!("{}", number);
}

fn if_palindrome(mut x: u64) -> bool {
    let mut digits: Vec<u64> = vec![];
    while x != 0 {
        digits.push(x % 10);
        x /= 10;
    }
    while digits.len() != 0 {
        if digits[0] != digits[digits.len()-1] {
            return false
        } else if digits.len() == 1 {
            digits.remove(0);
        } else {
            digits.remove(0);
            digits.remove(digits.len()-1);
        }
    }
    true
}

fn if_3dig_fact(x: u64) -> bool {
    for i in 100..1000 {
        if (x % i == 0) & (100 <= (x / i)) & ((x / i) < 1000) {
            return true
        }
    }
    false
}