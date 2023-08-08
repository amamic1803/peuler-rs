fn main() {
    let mut sum: u64 = 0;
    let mut values: [u64; 2] = [1, 2];
    while values[1] < 4000000 {
        if values[1] % 2 == 0 {
            sum += values[1];
        }
        let new_num: u64 = values[0] + values[1];
        values[0] = values[1];
        values[1] = new_num;
    }
    println!("{}", sum);
}
