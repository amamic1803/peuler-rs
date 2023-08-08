fn main() {

    let mut max_consecutive: i64 = 0;
    let mut product_of_max_consec: i64 = 0;

    for a in -999..1000 {
        for b in -999..1000 {
            let mut consecutive_primes: i64 = 0;
            let mut n: i64 = 0;
            loop {
                if is_prime((n * n) + (a * n) + b) {
                    consecutive_primes += 1;
                    n += 1;
                } else {
                    break;
                }
            }

            if consecutive_primes > max_consecutive {
                max_consecutive = consecutive_primes;
                product_of_max_consec = a * b;
            }

        }
    }
    println!("{}", product_of_max_consec);
}


fn is_prime(x: i64) -> bool {
    return if x < 2 {
        false
    } else if x == 2 {
        true
    } else {
        for i in (3..=((x as f64).sqrt() as i64)).step_by(2) {
            if x % i == 0 {
                return false
            }
        }
        true
    }
}