fn main() {
    let mut result: u64 = 0;
    for n in 1..1_000_000 {
        if (is_palindrome(n)) & (is_palindrome(format!("{n:b}").parse().unwrap())) {
            result += n;
        }
    }
    println!("{}", result);
}

fn is_palindrome(mut x: u64) -> bool {
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