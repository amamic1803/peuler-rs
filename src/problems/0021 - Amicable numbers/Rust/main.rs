use std::collections::HashMap;

fn main() {
    let mut sum_of_divisors = HashMap::new();

    for n in 1..10_000 {
        sum_of_divisors.insert(n, sum_divisors(n));
    }

    let mut sum_of_amicables: u64 = 0;

    for (number, summed_divisors) in &sum_of_divisors {
        if (summed_divisors < &10_000) & (summed_divisors != &0) & (number != summed_divisors) {
            if number == &sum_of_divisors[&summed_divisors] {
                sum_of_amicables += number;
            }
        }
    }
    println!("{}", sum_of_amicables);
}

fn sum_divisors(x: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=(x / 2) {
        if x % i == 0 {
            sum += i;
        }
    }
    sum
}