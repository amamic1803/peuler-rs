fn main() {
    let mut abundant_numbers: Vec<u64> = vec![];
    for n in 1..=28123 {
        if sum_divisors(n) > n {
            abundant_numbers.push(n);
        }
    }

    let mut result: u64 = 0;

    for number in 1..=28123 {
        let mut cant_be_sum: bool = true;

        'outer: for i in 0..abundant_numbers.len() {
            for j in i..abundant_numbers.len() {
                if number == (abundant_numbers[i] + abundant_numbers[j]) {
                    cant_be_sum = false;
                    break 'outer;
                }
            }
        }

        if cant_be_sum == true {
            result += number;
        }

    }

    println!("{}", result);

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
