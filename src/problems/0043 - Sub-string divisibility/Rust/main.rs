fn main() {
    let all_digits: Vec<i64> = (0..10).collect();
    let mut sum: i64 = 0;
    for permutation in quickperm(all_digits) {
        if (permutation[0] != 0) &&
            (vec_to_int(permutation[1..4].to_vec()) % 2 == 0) &&
            (vec_to_int(permutation[2..5].to_vec()) % 3 == 0) &&
            (vec_to_int(permutation[3..6].to_vec()) % 5 == 0) &&
            (vec_to_int(permutation[4..7].to_vec()) % 7 == 0) &&
            (vec_to_int(permutation[5..8].to_vec()) % 11 == 0) &&
            (vec_to_int(permutation[6..9].to_vec()) % 13 == 0) &&
            (vec_to_int(permutation[7..10].to_vec()) % 17 == 0) {
            sum += vec_to_int(permutation);
        }
    }
    println!("{}", sum);
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

fn vec_to_int(vector: Vec<i64>) -> i64 {
    let mut integer: i64 = 0;
    for ind in 0..vector.len() {
        integer += vector[ind] * 10_i64.pow((vector.len() - 1 - ind) as u32);
    }
    integer
}