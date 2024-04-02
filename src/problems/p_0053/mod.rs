//! **Problem 53** - *Combinatoric Selections*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(53, "Combinatoric Selections", solve)
}

use malachite::num::basic::traits::One;
use malachite::Natural;
use once_cell::sync::Lazy;

const LOW_N: usize = 23;
const N: usize = 100;
const LIMIT: usize = 1_000_000;

fn solve() -> String {
    let mut result = 0;
    let limit = Natural::from(LIMIT);

    for n in LOW_N..(N + 1) {
        let limit_r = (n >> 1) + 1;
        for r in 0..limit_r {
            let value = &FACT_CACHE[n] / (&FACT_CACHE[r] * &FACT_CACHE[n - r]);
            if value > limit {
                result += (limit_r - r) << 1;
                if n % 2 == 0 {
                    result -= 1;
                }
                break;
            }
        }
    }

    result.to_string()
}

static FACT_CACHE: Lazy<[Natural; N + 1]> = Lazy::new(|| {
    let mut cache = [Natural::ONE; N + 1];
    let mut current_num = Natural::ONE;

    for (i, position) in cache.iter_mut().enumerate().skip(2) {
        current_num *= Natural::from(i);
        *position = current_num.clone();
    }

    cache
});
