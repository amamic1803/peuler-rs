fn main() {
    let mut result: u64 = 0;
    let digits: Vec<u64> = (1..10).collect();
    let mut found_products: Vec<u64> = vec![];

    for a in 1..100 {
        for b in a..10_000 {
            let product: u64 = a * b;

            if !found_products.contains(&product) {
                let numbers: Vec<u64> = vec![a, b, product];
                if is_pandigital(&numbers, &digits) {
                    result += product;
                    found_products.push(product);
                }
            }

        }
    }

    println!("{}", result);
}


fn is_pandigital(numbers: &Vec<u64>, digits: &Vec<u64>) -> bool {
    let mut found_digits: Vec<u64> = vec![];

    for index in 0..numbers.len() {
        let mut temp_n = numbers[index];
        while temp_n != 0 {
            found_digits.push(temp_n % 10);
            temp_n /= 10;
        }
    }

    found_digits.sort();

    return if digits == &found_digits {
        true
    } else {
        false
    }
}