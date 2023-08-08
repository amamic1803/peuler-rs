fn main() {
    let mut max_recurring_digits: u64 = 0;
    let mut max_number: u64 = 0;

    for d in 1..1000 {
        let mut working_num: u64 = 1;
        let mut result: Vec<u64> = vec![];

        let mut recurring: bool = false;

        while working_num != 0 {
            working_num %= d;

            if result.contains(&working_num) {
                recurring = true;
                result.push(working_num);
                break;
            }
            result.push(working_num);
            working_num *= 10;

        }

        if recurring {
            let mut length_recurring: u64 = 0;
            for i in 0..(result.len()) {
                if (i != (result.len() - 1)) & (result[i] == result[result.len() - 1]) {
                    length_recurring = ((result.len() - 1) - i) as u64;
                }
            }

            if length_recurring > max_recurring_digits {
                max_recurring_digits = length_recurring;
                max_number = d;
            }

        }

    }

    println!("{}", max_number);
}
