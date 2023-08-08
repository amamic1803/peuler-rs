fn main() {
    let mut maximum_seq: u64 = 0;
    let mut maximum_seq_num: u64 = 0;
    for number in 1..1_000_000 {
        let seq_len: u64 = collatz_seq_len(number);
        if seq_len > maximum_seq {
            maximum_seq = seq_len;
            maximum_seq_num = number;
        }
    }
    println!("{}", maximum_seq_num);
}

fn collatz_seq_len(mut num: u64) -> u64 {
    let mut seq_len: u64 = 0;
    loop {
        if num == 1 {
            seq_len += 1;
            break;
        } else if num % 2 == 0 {
            num /= 2;
            seq_len += 1;
        } else {
            num = (3 * num) + 1;
            seq_len += 1
        }
    }
    seq_len
}