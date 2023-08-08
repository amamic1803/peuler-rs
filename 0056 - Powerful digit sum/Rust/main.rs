// Add to Cargo.toml
// [dependencies]
// num-bigint = "0.4.3"
// num-traits = "0.2.15"

use num_bigint::{BigUint, ToBigUint};
use num_traits::cast::ToPrimitive;

fn main() {
    let mut max_sum: u64 = 0;
    for a in 1..100 {
        for b in 1..100 {
            let digits_sum: u64 = sum_of_digits(a.to_biguint().unwrap().pow(b));
            if digits_sum > max_sum {max_sum = digits_sum;}
        }
    }
    println!("{}", max_sum);
}

fn sum_of_digits(mut n: BigUint) -> u64 {
    let mut digits_sum: u64 = 0;
    while n != 0.to_biguint().unwrap() {
        digits_sum += (&n % 10.to_biguint().unwrap()).to_u64().unwrap();
        n /= 10.to_biguint().unwrap();
    }
    digits_sum
}
