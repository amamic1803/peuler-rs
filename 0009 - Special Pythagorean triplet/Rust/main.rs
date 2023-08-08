fn main() {
    for a in 1..=997 {
        if (500_000 - (1_000 * a)) % (1000 - a) == 0 {
            let b: u64 = (500_000 - (1_000 * a)) / (1000 - a);
            let result: u64 = a * b * (1000 - a - b);
            println!("{}", result);
            break;
        }
    }
}
