fn main() {
    let mut result: [u64; 2] = [1, 1];

    for i in 10..100 {
        for j in (i + 1)..100 {
            let numerator: u64 = i;
            let denominator: u64 = j;
            let mut digits_numerator: Vec<u64> = vec![numerator / 10, numerator % 10];
            let mut digits_denominator: Vec<u64> = vec![denominator / 10, denominator % 10];
            'outer: for m in 0..digits_numerator.len() {
                for n in 0..digits_denominator.len() {
                    if (digits_numerator[m] == digits_denominator[n]) & (digits_numerator[m] != 0) {
                        digits_numerator.remove(m);
                        digits_denominator.remove(n);
                        let lowest_terms = lowest_common_terms([digits_numerator[0], digits_denominator[0]]);
                        if lowest_common_terms([numerator, denominator]) == lowest_terms {
                            result[0] *= lowest_terms[0];
                            result[1] *= lowest_terms[1];
                        }
                        break 'outer;
                    }
                }
            }

        }
    }
    result = lowest_common_terms(result);
    println!("{}", result[1]);
}

fn lowest_common_terms(fraction: [u64; 2]) -> [u64; 2] {
    let mut working: Vec<u64> = fraction.to_vec();
    working.sort();
    working.reverse();

    while working[working.len() - 1] > 1 {
        working.push(working[0] % working[1]);
        working.remove(0);
    }

    return if working[1] == 0 {
        [fraction[0] / working[0], fraction[1] / working[0]]
    } else {
        fraction
    }

}