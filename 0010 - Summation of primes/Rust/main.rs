fn main() {
    let mut sum: u64 = 2;
    let mut curr_num: u64 = 3;
    while curr_num < 2_000_000 {
        if is_prime(curr_num) {
            sum += curr_num;
        }
        curr_num += 2;
    }
    println!("{}", sum);
}

fn is_prime(x: u64) -> bool {
    for i in (3..=((x as f64).sqrt() as u64)).step_by(2) {
        if x % i == 0 {
            return false
        }
    }
    true
}