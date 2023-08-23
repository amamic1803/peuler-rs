fn main() {
    let mut circular_nums: i64 = 0;
    for i in 1..1_000_000 {
        if is_circular_prime(i) {
            circular_nums += 1;
        }
    }
    println!("{}", circular_nums);
}

fn is_circular_prime(mut n: i64) -> bool {
    return if is_prime(n) {
        let len_of_n: i64 = len_of_number(n);
        let mut all_primes: bool = true;

        for _ in 1..len_of_n {
            n = ((n % (10_i64.pow((len_of_n - 1) as u32))) * 10) + (n / (10_i64.pow((len_of_n - 1) as u32)));
            if !is_prime(n) {
                all_primes = false;
                break;
            }
        }

        all_primes
    } else {
        false
    }
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

fn len_of_number(mut n: i64) -> i64 {
    let mut result: i64 = 0;
    while n != 0 {
        result += 1;
        n /= 10;
    }
    result
}