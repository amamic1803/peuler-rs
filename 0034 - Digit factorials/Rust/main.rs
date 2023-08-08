fn main() {
    let mut result: u64 = 0;
    for n in 10..(2_540_160 + 1) {
        if n == sum_of_digits_factorial(n) {
            result += n;
        }
    }
    println!("{}", result);
}

fn sum_of_digits_factorial(mut number: u64) -> u64 {
    let mut sum: u64 = 0;
    while number != 0 {
        sum += factorial(number % 10);
        number /= 10;
    }
    sum
}

fn factorial(number: u64) -> u64 {
    return if number == 0 {
        1
    } else {
        let mut fact: u64 = 1;
        for i in 1..(number + 1) {
            fact *= i;
        }
        fact
    }
}