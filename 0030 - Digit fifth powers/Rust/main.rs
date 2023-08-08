fn main() {
    let mut result_sum: u64 = 0;

    // 354_294 = 6 * 9^5
    for n in 10..354_295 {
        let mut sum_of_digits: u64 = 0;
        let mut working_number: u64 = n;
        while working_number != 0 {
            sum_of_digits += (working_number % 10).pow(5);
            working_number /= 10;
        }
        if sum_of_digits == n {
            result_sum += sum_of_digits;
        }
    }
    println!("{}", result_sum);
}
