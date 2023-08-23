fn main() {
    let mut left_digits: Vec<u64> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut left_places: u64 = 999_999;
    let mut result = String::new();

    while left_places != 0 {
        let fact: u64 = factorial((left_digits.len() as u64) - 1);
        let index: u64 = left_places / fact;
        result.push_str(&left_digits[index as usize].to_string());
        left_digits.remove(index.try_into().unwrap());
        left_digits.sort();
        left_places -= index * fact;
    }

    for i in left_digits {
        result.push_str(&i.to_string());
    }

    println!("{}", result);
}


fn factorial(x: u64) -> u64 {
    return if x == 0 {
        1
    } else {
        let mut result: u64 = 1;
        for i in 1..=x {
            result *= i;
        }
        result
    }
}