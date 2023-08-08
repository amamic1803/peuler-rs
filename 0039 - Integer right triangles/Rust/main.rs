use std::collections::HashMap;

fn main() {
    let mut occurences = HashMap::new();

    for i in 1_u64..1001 {
        for j in i..1001 {
            for z in j..1001 {
                if ((i.pow(2) + j.pow(2) == z.pow(2)) | (z.pow(2) + j.pow(2) == i.pow(2)) | (i.pow(2) + z.pow(2) == j.pow(2))) & (i + j + z < 1001) {
                    *occurences.entry(i + j + z).or_insert(0) += 1;
                }
            }
        }
    }

    let mut max = 0;
    let mut result = 0;
    for (key, value) in &occurences {
        if value > &max {
            max = *value;
            result = *key;
        }
    }
    println!("{}", result);
}
