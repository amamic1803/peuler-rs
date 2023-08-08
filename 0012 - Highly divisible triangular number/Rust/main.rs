fn main() {
    let mut n: u64 = 1;
    let mut curr_triang_num: u64 = 1;
    loop {
        if num_of_divisors(curr_triang_num) < 500 {
            n += 1;
            curr_triang_num += n;
        } else {
            break;
        }
    }
    println!("{}", curr_triang_num);
}

fn num_of_divisors(number: u64) -> u64 {
    let mut count: u64 = 0;
    let end = (number as f64).sqrt() as u64;
    for i in 1..=end {
        if number % i == 0 {
            count += 2;
        }
    }
    if end.pow(2) == number {
        count -= 1;
    }
    count
}