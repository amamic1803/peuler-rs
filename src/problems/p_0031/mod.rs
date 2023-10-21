//! **Problem 31** - *Coin Sums*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        31,
        "Coin Sums",
        solve,
    )
}


use std::collections::HashMap;

fn solve() -> String {
    let coins: [u64; 8] = [200, 100, 50, 20, 10, 5, 2, 1];  // available coins
    let mut cache: HashMap<(u64, usize), u64> = HashMap::new();  // used to memoize results
    let wanted_value: u64 = 200;  // 2£ = 200p

    num_of_combinations(wanted_value, 0, &coins, &mut cache).to_string()
}

fn num_of_combinations(left_money: u64, coin_ind: usize, coins: &[u64], cache: &mut HashMap<(u64, usize), u64>) -> u64 {
    cache.get(&(left_money, coin_ind)).copied().unwrap_or_else(|| {
        if left_money == 0 {
            1
        } else if coin_ind == coins.len() {
            0
        } else {
            let mut sum_of_combinations = 0;

            for k in 0..((left_money / coins[coin_ind]) + 1) {
                sum_of_combinations += num_of_combinations(left_money - (k * coins[coin_ind]), coin_ind + 1, coins, cache);
            }

            cache.insert((left_money, coin_ind), sum_of_combinations);
            sum_of_combinations
        }
    })
}
