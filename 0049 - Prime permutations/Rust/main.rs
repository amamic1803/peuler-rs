fn main() {
    let mut results: Vec<String> = vec![];
    for i in 1..10 {
        for j in i..10 {
            for z in j..10 {
                for y in z..10 {
                    let mut all_permutations: Vec<Vec<i64>> = quickperm(vec![i, j, z ,y]);
                    let mut deleted = 0;
                    for ind in 0..all_permutations.len() {
                        if !is_prime(vec_to_int(&all_permutations[ind - deleted])) {
                            all_permutations.remove(ind - deleted);
                            deleted += 1;
                        }
                    }
                    if all_permutations.len() >= 3 {
                        all_permutations.sort();
                        'perms: for permutation_ind in 0..(all_permutations.len() - 2) {
                            for other_perm_ind in permutation_ind..(all_permutations.len() - 1) {
                                if all_permutations[permutation_ind] != all_permutations[other_perm_ind]
                                    && all_permutations.contains(&int_to_vec((vec_to_int(&all_permutations[other_perm_ind]) - vec_to_int(&all_permutations[permutation_ind])) * 2 + vec_to_int(&all_permutations[permutation_ind]))) {
                                    let mut out_str: String = String::new();
                                    out_str.push_str(&vec_to_int(&all_permutations[permutation_ind]).to_string());
                                    out_str.push_str(&vec_to_int(&all_permutations[other_perm_ind]).to_string());
                                    out_str.push_str(&((vec_to_int(&all_permutations[other_perm_ind]) - vec_to_int(&all_permutations[permutation_ind])) * 2 + vec_to_int(&all_permutations[permutation_ind])).to_string());
                                    results.push(out_str);
                                    break 'perms;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", results[1]);
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

fn vec_to_int(vector: &Vec<i64>) -> i64 {
    let mut integer: i64 = 0;
    for ind in 0..vector.len() {
        integer += vector[ind] * 10_i64.pow((vector.len() - 1 - ind) as u32);
    }
    integer
}

fn int_to_vec(mut int: i64) -> Vec<i64> {
    let mut output: Vec<i64> = vec![];
    while int != 0 {
        output.insert(0, int % 10);
        int /= 10;
    }
    output
}