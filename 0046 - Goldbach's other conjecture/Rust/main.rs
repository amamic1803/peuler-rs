fn main() {
    let mut list_of_primes: Vec<u64> = vec![2];
    let mut current_num: u64 = 3;
    loop {
        if is_prime(current_num as i64) {
            list_of_primes.push(current_num);
        } else {
            let mut is_part_of_seq: bool = false;
            for prime in &list_of_primes {
                if (((current_num as f64) - (*prime as f64)) / 2.0).sqrt() % 1.0 == 0.0 {
                    is_part_of_seq = true;
                    break;
                }
            }
            if !is_part_of_seq {
                println!("{}", current_num);
                break;
            }
        }
        current_num += 2;
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