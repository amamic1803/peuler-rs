// add to Cargo.toml under dependencies:
// num-bigint = "0.4"

use num_bigint::{BigUint, ToBigUint};

fn main() {
    let mut result: u64 = 0;

    let mut n_fact: BigUint = factorial(22);
    for n in 23..101 {
        n_fact *= n.to_biguint().unwrap();
        let mut r: u64 = n / 2;
        let mut n_min_r: u64 = n - r;
        let mut r_fact: BigUint = factorial(r);
        let mut n_min_r_fact: BigUint = factorial(n_min_r);

        while &n_fact / (&r_fact * &n_min_r_fact) > 1_000_000.to_biguint().unwrap() {
            result += 2;
            r_fact /= r.to_biguint().unwrap();
            r -= 1;
            n_min_r += 1;
            n_min_r_fact *= n_min_r.to_biguint().unwrap();
        }
        if n % 2 == 0 {result -= 1;}
    }

    println!("{}", result);
}

fn factorial(n: u64) -> BigUint {
    let mut result: BigUint = 1.to_biguint().unwrap();
    for i in 2..(n + 1) {
        result *= i.to_biguint().unwrap();
    }
    return result
}
