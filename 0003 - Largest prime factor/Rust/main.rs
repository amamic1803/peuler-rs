fn main() {
    let mut given_num: u64 = 600851475143;
    let mut is_prime: bool = false;
    while !is_prime {
        for i in 2..=given_num {
            if i == given_num {
                is_prime = true;
            } else if given_num % i == 0 {
                given_num /= i;
                break;
            }
        }
    }
    println!("{}", given_num);
}
