use std::collections::HashMap;

fn main() {
    //february - 28 - 29 (leap year, divisible by 4 or 400 if century)
    //april, june, september, november - 30
    //january, march, may, july, august, october, december - 31
    //1.1.1900 was Monday
    let mut days_in_month = HashMap::new();

    days_in_month.insert(1, 31);
    days_in_month.insert(2, 28);
    days_in_month.insert(3, 31);
    days_in_month.insert(4, 30);
    days_in_month.insert(5, 31);
    days_in_month.insert(6, 30);
    days_in_month.insert(7, 31);
    days_in_month.insert(8, 31);
    days_in_month.insert(9, 30);
    days_in_month.insert(10, 31);
    days_in_month.insert(11, 30);
    days_in_month.insert(12, 31);

    let mut current_year: u64 = 1900;
    let mut current_month: u64 = 1;
    let mut current_day: u64 = 1;

    let mut sundays: u64 = 0;

    while current_year <= 2000 {
        if (current_day % 7 == 0) & (current_year != 1900) {
            sundays += 1;
        }

        current_day += days_in_month[&current_month];

        if current_month == 2 {
            if (current_year % 100 == 0) & (current_year % 400 == 0) {
                current_day += 1;
            } else if current_year % 4 == 0 {
                current_day += 1;
            }
        } else if current_month == 12 {
            current_month = 0;
            current_year += 1;
        }
        current_month += 1;
    }

    println!("{}", sundays);
}
