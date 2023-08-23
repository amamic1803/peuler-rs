fn main() {
    let mut decimal_part: Vec<u64> = vec![];
    let mut curr_num: u64 = 0;
    while decimal_part.len() < 1_000_000 {
        curr_num += 1;
        let mut temp_num: u64 = curr_num;
        let index = decimal_part.len();
        while temp_num != 0 {
            decimal_part.insert(index, temp_num % 10);
            temp_num /= 10;
        }
    }
    println!("{}", (decimal_part[1 - 1] * decimal_part[10 - 1] * decimal_part[100 - 1] * decimal_part[1_000 - 1] * decimal_part[10_000 - 1] * decimal_part[100_000 - 1] * decimal_part[1_000_000 - 1]));
}
