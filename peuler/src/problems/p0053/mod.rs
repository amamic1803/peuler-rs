use crate::Solution;
use malachite::Natural;
use malachite::base::num::basic::traits::One;
use std::sync::LazyLock;

problem!(Problem0053, 53, "Combinatoric Selections");

impl Solution for Problem0053 {
    fn solve(&self) -> String {
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
}

const LOW_N: usize = 23;
const N: usize = 100;
const LIMIT: usize = 1_000_000;

static FACT_CACHE: LazyLock<[Natural; N + 1]> = LazyLock::new(|| {
    let mut cache = [Natural::ONE; N + 1];
    let mut current_num = Natural::ONE;

    for (i, position) in cache.iter_mut().enumerate().skip(2) {
        current_num *= Natural::from(i);
        *position = current_num.clone();
    }

    cache
});
