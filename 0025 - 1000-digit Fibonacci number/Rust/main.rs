fn main() {
    let mut fibonacci: Vec<Vec<u64>> = vec![vec![1], vec![1]];
    let mut fibonacci_index: u64 = 2;

    while fibonacci[1].len() < 1000 {
        let mut result: Vec<u64> = vec![];

        let mut transfer: u64 = 0;
        for j in (0..fibonacci[1].len()).rev() {
            let mut working_sum: u64 = transfer;
            for i in 0..2 {
                working_sum += fibonacci[i][j];
            }
            result.insert(0, working_sum % 10);
            transfer = working_sum / 10;
        }
        while transfer != 0 {
            result.insert(0, transfer % 10);
            transfer /= 10;
        }

        fibonacci.push(result);
        fibonacci.remove(0);

        for _ in 0..(fibonacci[1].len() - fibonacci[0].len()) {
            fibonacci[0].insert(0, 0);
        }

        fibonacci_index += 1;
    }

    println!("{}", fibonacci_index);

}
