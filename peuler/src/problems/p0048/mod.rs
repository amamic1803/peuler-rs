use crate::Solution;

problem!(Problem0048, 48, "Self Powers");

impl Solution for Problem0048 {
    fn solve(&self) -> String {
        // full manual calculation method commented out below, this is better
        // by using mod at every step we discard unnecessary digits

        let mut result: u64 = 0;

        for n in 1..1001 {
            result += power_mod(n, n);
            result %= MODULO;
        }

        result.to_string()
    }
}

const MODULO: u64 = 10_000_000_000;

fn power_mod(base: u64, exponent: u64) -> u64 {
    // calculates base^exponent % MODULO
    let mut result = 1;
    for _ in 0..exponent {
        result *= base;
        result %= MODULO;
    }
    result
}

// fn solve() {
//     let mut result: Vec<u64> = vec![];
//     for i in 1..1001 {
//         let mut working_num: Vec<u64> = vec![];
//         let mut temp_num: u64 = i;
//         while temp_num != 0 {
//             working_num.insert(0, temp_num % 10);
//             temp_num /= 10;
//         }
//         let mut resulting_num: Vec<u64> = working_num.clone();
//         for _ in 0..(i - 1) {
//             multiply_num(&mut resulting_num, &mut working_num);
//         }
//         sum_nums(&mut result, &mut resulting_num);
//     }
//     let mut output: String = String::new();
//     let mut starting_point = 0;
//     if result.len() > 10 {
//         starting_point = result.len() - 10;
//     }
//     for i in starting_point..result.len() {
//         output.push_str(&result[i].to_string());
//     }
//     println!("{}", output);
// }
//
// fn sum_nums(num_to_add_to: &mut Vec<u64>, num_to_add: &mut Vec<u64>) {
//     let mut leftover: u64 = 0;
//     let mut num_from_back = 1;
//     loop {
//         let mut sum: u64 = leftover;
//         if num_to_add.len() >= num_from_back {
//             sum += num_to_add[num_to_add.len() - num_from_back];
//         }
//         let len_of_num_to_add_to = num_to_add_to.len();
//         if len_of_num_to_add_to >= num_from_back {
//             sum += num_to_add_to[len_of_num_to_add_to - num_from_back];
//             num_to_add_to[len_of_num_to_add_to - num_from_back] = sum % 10;
//         } else {
//             num_to_add_to.insert(0, sum % 10);
//         }
//         leftover = sum / 10;
//         if (num_to_add.len() <= num_from_back)
//             && (num_to_add_to.len() <= num_from_back)
//             && (leftover == 0)
//         {
//             break;
//         }
//         num_from_back += 1;
//     }
// }
//
// fn multiply_num(num_to_multiply: &mut Vec<u64>, multiplier: &mut Vec<u64>) {
//     let mut factoring: Vec<Vec<u64>> = vec![];
//     for factoring_num in (0..multiplier.len()).rev() {
//         let mut current_product: Vec<u64> = vec![];
//         let mut transfer: u64 = 0;
//         for j in (0..num_to_multiply.len()).rev() {
//             let working_product: u64 = (num_to_multiply[j] * multiplier[factoring_num]) + transfer;
//             current_product.insert(0, working_product % 10);
//             transfer = working_product / 10;
//         }
//         while transfer != 0 {
//             current_product.insert(0, transfer % 10);
//             transfer /= 10;
//         }
//         factoring.push(current_product);
//     }
//     for i in 0..factoring.len() {
//         for _ in 0..i {
//             factoring[i].push(0)
//         }
//     }
//     *num_to_multiply = vec![];
//     for mut number_for_sum in factoring {
//         sum_nums(num_to_multiply, &mut number_for_sum);
//     }
// }
//
