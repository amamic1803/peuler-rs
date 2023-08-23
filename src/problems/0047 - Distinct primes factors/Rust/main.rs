fn main() {
    let how_many_consecutive: i64 = 4;
    let mut consecutive: Vec<i64> = vec![0; how_many_consecutive.try_into().unwrap()];
    let mut curr_num: i64 = 2;

    loop {
        if distinct_prime_factors(curr_num) == how_many_consecutive {
            consecutive.push(curr_num);
            consecutive.remove(0);
        }
        curr_num += 1;
        if sum_vector(&consecutive) == (how_many_consecutive * consecutive[0] + (((how_many_consecutive - 1) * how_many_consecutive) / 2)) {
            break;
        }
    }

    println!("{}", consecutive[0]);
}

fn sum_vector(vector: &Vec<i64>) -> i64 {
    let mut sum: i64 = 0;
    for num in vector {
        sum += *num;
    }
    sum
}

fn first_prime_fact(x: i64) -> i64 {
    return if x < 2 {
        x
    } else if x == 2 {
        x
    } else if x % 2 == 0 {
        2
    } else {
        for i in (3..=((x as f64).sqrt() as i64)).step_by(2) {
            if x % i == 0 {
                return i
            }
        }
        x
    }
}

fn distinct_prime_factors(mut x: i64) -> i64 {
    let mut distinct_factors: i64 = 0;
    let mut distinct_factors_working: i64 = 1;
    while x != 1 {
        let first_prime_factor: i64 = first_prime_fact(x);
        if distinct_factors_working % first_prime_factor != 0 {
            distinct_factors += 1;
            distinct_factors_working *= first_prime_factor;
        }
        x /= first_prime_factor;
    }
    distinct_factors
}
