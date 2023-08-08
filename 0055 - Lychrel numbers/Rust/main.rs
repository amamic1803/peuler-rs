fn main() {
    let mut result: u64 = 0;
    for n in 10..10_000 {
        if is_lychrel(n, 50) {result += 1;}
    }
    println!("{}", result);
}


fn is_lychrel(mut num: u128, iterations: u128) -> bool {
    let mut lychrel: bool = true;
    num += invert_num(num);
    for _ in 1..iterations {
        let num_inv: u128 = invert_num(num);
        if num == num_inv {
            lychrel = false;
            break;
        } else {
            num += num_inv;
        }
    }
    lychrel
}

fn invert_num(mut num: u128) -> u128 {
    let mut result: u128 = 0;
    while num != 0 {
        result = (result * 10) + (num % 10);
        num /= 10;
    }
    result
}