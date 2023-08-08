fn main() {
    let mut largest_found: i64 = 0;
    for curr_len in 1..10 {
        for x in quickperm((1..(curr_len + 1)).collect()) {
            let integer = vec_to_int(x);
            if is_prime(integer) && (integer > largest_found) {
                largest_found = integer;
            }
        }
    }
    println!("{}", largest_found);
}

fn is_prime(x: i64) -> bool {
    return if x < 2 {
        false
    } else if x == 2 {
        true
    } else if x % 2 == 0 {
        false
    } else {
        for i in (3..=((x as f64).sqrt() as i64)).step_by(2) {
            if x % i == 0 {
                return false
            }
        }
        true
    }
}

fn vec_to_int(vector: Vec<i64>) -> i64 {
    let mut integer: i64 = 0;
    for ind in 0..vector.len() {
        integer += vector[ind] * 10_i64.pow((vector.len() - 1 - ind) as u32);
    }
    integer
}

fn quickperm(mut a: Vec<i64>) -> Vec<Vec<i64>> {
    let mut permutations: Vec<Vec<i64>> = vec![a.clone()];
    let n = a.len();
    let mut p: Vec<u8> = vec![0; n];
    let mut i: usize = 1;
    while i < n {
        if (p[i] as usize) < i {
            let j: usize = match i % 2 {
                1 => p[i] as usize,
                _ => 0,
            };
            let temp_val = a[i];
            a[i] = a[j];
            a[j] = temp_val;
            permutations.push(a.clone());
            p[i] += 1;
            i = 1;
        } else {
            p[i] = 0;
            i += 1;
        }
    }
    permutations
}