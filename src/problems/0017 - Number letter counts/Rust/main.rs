use std::collections::HashMap;

fn main() {
    let mut single_dig = HashMap::new();
    single_dig.insert(1, "one");
    single_dig.insert(2, "two");
    single_dig.insert(3, "three");
    single_dig.insert(4, "four");
    single_dig.insert(5, "five");
    single_dig.insert(6, "six");
    single_dig.insert(7, "seven");
    single_dig.insert(8, "eight");
    single_dig.insert(9, "nine");

    let mut double_dig = HashMap::new();
    double_dig.insert(2, "twenty");
    double_dig.insert(3, "thirty");
    double_dig.insert(4, "forty");
    double_dig.insert(5, "fifty");
    double_dig.insert(6, "sixty");
    double_dig.insert(7, "seventy");
    double_dig.insert(8, "eighty");
    double_dig.insert(9, "ninety");

    let mut teen_dig = HashMap::new();
    teen_dig.insert(10, "ten");
    teen_dig.insert(11, "eleven");
    teen_dig.insert(12, "twelve");
    teen_dig.insert(13, "thirteen");
    teen_dig.insert(14, "fourteen");
    teen_dig.insert(15, "fifteen");
    teen_dig.insert(16, "sixteen");
    teen_dig.insert(17, "seventeen");
    teen_dig.insert(18, "eighteen");
    teen_dig.insert(19, "nineteen");

    let mut number_of_letters: u64 = 0;

    for i in 1..=1_000 {
        let mut number: u64 = i;
        let mut string_name = String::new();
        if number == 1000 {
            string_name.push_str(&"onethousand");
        } else {
            let hundreds: u64 =  number / 100;
            if hundreds != 0 {
                string_name.push_str(&single_dig[&hundreds]);
                string_name.push_str(&"hundred")
            }

            number %= 100;
            if number != 0 {
                if hundreds != 0 {
                    string_name.push_str(&"and");
                }
                if (9 < number) & (number < 20) {
                    string_name.push_str(&teen_dig[&number]);
                } else if number < 10 {
                    string_name.push_str(&single_dig[&number]);
                } else {
                    let tens: u64 = number / 10;
                    let ones: u64 = number % 10;
                    string_name.push_str(&double_dig[&tens]);
                    if ones != 0 {
                        string_name.push_str(&single_dig[&ones]);
                    }
                }
            }
        }

        number_of_letters += string_name.len() as u64;

    }

    println!("{}", number_of_letters);
}