fn main() {
    let mut working_num: Vec<u64> = vec![1];
    let mut transfer: u64 = 0;
    for _ in 1..=1000 {
        for j in (0..working_num.len()).rev() {
            let working_product: u64 = (working_num[j] * 2) + transfer;
            working_num[j] = working_product % 10;
            transfer = working_product / 10;
        }
        while transfer != 0 {
        working_num.insert(0, transfer % 10);
        transfer /= 10;
        }
    }


    let mut sum: u64 = 0;
    for i in &working_num {
        sum += i;
    }

    println!("{}", sum);
}
