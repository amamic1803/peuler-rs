fn main() {
    let coins: [u64; 8] = [200, 100, 50, 20, 10, 5, 2, 1];
    let wanted_value: u64 = 200;  // 2£ = 200p

    println!("{}", number_of_combinations(wanted_value, &coins, 0));
}

fn number_of_combinations(left_money: u64, coins: &[u64; 8], current_coin: usize) -> u64 {
    return if left_money == 0 {
        1
    } else if current_coin == coins.len() {
        0
    } else {
        let mut sum_of_combinations: u64 = 0;
        for k in 0..((left_money / coins[current_coin]) + 1) {
            sum_of_combinations += number_of_combinations(left_money - (k * coins[current_coin]), coins, current_coin + 1);
        }
        sum_of_combinations
    }
}