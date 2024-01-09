//! **Problem 78** - *Coin Partitions*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        78,
        "Coin Partitions",
        solve,
    )
}

const DIVISOR: u32 = 1_000_000;

fn solve() -> String {
    // partition_p function rises too quickly so it won't work for this problem
    // instead we will use it's modification here that doesn't actually store whole partitions, but partition mod 1_000_000
    // that is enough to know if the partition is divisible by 1_000_000 or not
    // and since new partitions are calculated from previous ones with basically just addition, we don't need info about whole partitions

    // initialize partition cache, p(0) = 1, p(1) = 1
    let mut cache = vec![1, 1];

    // while last value in cache is not 0 (i.e. last partition is not divisible by 1_000_000)
    while cache[cache.len() - 1] != 0 {
        // calculate next value (mod) and add it to vector
        let n = cache.len();
        let mut next_val = 0;
        for k in 1..(n + 1) {
            let left_value = match n.checked_sub((k * (3 * k - 1)) >> 1) {
                Some(ind) => cache[ind],
                None => break,  // larger of the indices is below zero, so any larger k will only be 0, we can break
            };
            let right_value = match n.checked_sub((k * (3 * k + 1)) >> 1) {
                Some(ind) => cache[ind],
                None => 0,
            };
            let value = (left_value + right_value) % DIVISOR;

            if k % 2 == 0 {
                next_val = (next_val + (DIVISOR - value)) % DIVISOR;  // sum with complement == subtract
            } else {
                next_val = (next_val + value) % DIVISOR;
            }
        }

        // push the newly calculated value to the cache vector
        cache.push(next_val);
    }

    // return the index of the last value in cache
    (cache.len() - 1).to_string()
}
