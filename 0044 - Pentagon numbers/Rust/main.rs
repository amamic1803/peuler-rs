fn main() {
    let mut sequence: Vec<u64> = vec![1, 5];
    let mut curr_diff: u64 = 7;
    let mut curr_ind: usize = 1;

    'outer: loop {
        let sum = sequence[curr_ind] + sequence[curr_ind - 1];
        while sequence[sequence.len() - 1] < sum {
            sequence.push(sequence[sequence.len() - 1] + curr_diff);
            curr_diff += 3;
        }
        for i in 0..curr_ind {
            if sequence.contains(&(sequence[curr_ind] - sequence[i])) && sequence.contains(&(sequence[curr_ind] + sequence[i])) {
                println!("{}", sequence[curr_ind] - sequence[i]);
                break 'outer;
            }
        }
        curr_ind += 1;
    }
}