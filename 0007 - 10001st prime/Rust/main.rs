fn main() {
    let mut number_of_primes: u32 = 2;
    let mut curr_num: u64 = 3;
    while number_of_primes < 10001 {
        curr_num += 2;
        if is_prime(curr_num) {
            number_of_primes += 1;
        }
    }
    println!("{}", curr_num);
}

fn is_prime(x: u64) -> bool {
    for i in (3..=((x as f64).sqrt() as u64)).step_by(2) {
        if x % i == 0 {
            return false
        }
    }
    true
}