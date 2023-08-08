fn main() {
    let mut primes: Vec<i64> = vec![2];
    for i in (3..1_000_000).step_by(2) {
        if is_prime(i) {primes.push(i);}
    }
    let mut longest_consecutive_primes = 0;
    let mut num_with_most_consec_prim: i64 = 0;
    for i in &primes {
        let mut num: i64 = 0;
        let mut curr_ind = 0;
        loop {
            if num + primes[curr_ind] >= *i {
                break;
            } else {
                num += primes[curr_ind];
                curr_ind += 1;
            }
        }
        if num == *i {
            if longest_consecutive_primes < curr_ind {
                longest_consecutive_primes = curr_ind;
                num_with_most_consec_prim = *i;
            }
        } else {
            let mut curr_min_ind = 0;
            while (num != *i) && (curr_min_ind != curr_ind) {
                num += primes[curr_ind];
                curr_ind += 1;
                while num > *i {
                    num -= primes[curr_min_ind];
                    curr_min_ind += 1;
                }
            }
            if longest_consecutive_primes < (curr_ind - curr_min_ind) {
                longest_consecutive_primes = curr_ind - curr_min_ind;
                num_with_most_consec_prim = *i;
            }
        }
    }
    println!("{}", num_with_most_consec_prim);
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