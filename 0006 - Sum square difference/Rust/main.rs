fn main() {
    let mut sum_of_squares: i64 = 0;
    for i in 1..=100 as i64 {
        sum_of_squares += i.pow(2);
    }
    let result: i64 = (((100 * 101) / 2) as i64).pow(2) - sum_of_squares;
    println!("{}", result);
}
