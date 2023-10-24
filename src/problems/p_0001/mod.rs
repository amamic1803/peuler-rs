//! **Problem 1** - *Multiples of 3 or 5*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        1,
        "Multiples of 3 or 5",
        solve,
    )
}

const UPPER_LIMIT: usize = 1000;

fn solve() -> String {
    // sum of multiples of 3 looks like this:
    // 1*3 + 2*3 + 3*3 + 4*3 + 5*3 + ... + n*3
    // n*3 is the last multiple of 3 that is less than UPPER_LIMIT (1000)
    // we can find this n by dividing (UPPER_LIMIT - 1) (because we don't actually want to include UPPER_LIMIT) by 3 and rounding down
    // so n = (UPPER_LIMIT - 1) / 3
    // we can rewrite this sum as:
    // 3 * (1 + 2 + 3 + 4 + 5 + ... + n)
    // or:
    // 3 * n * (n + 1) / 2
    // similarly for multiples of 5:
    // 5 * (1 + 2 + 3 + 4 + 5 + ... + m)
    // or:
    // 5 * m * (m + 1) / 2
    // where m = (UPPER_LIMIT - 1) / 5
    // if we sum these expressions we will get the result that is higher than expected
    // that is because we are actually counting numbers that are multiples of both 3 and 5 twice
    // so we need to subtract the sum of multiples of 15 (multiples of 3 and 5)
    // for 15 we have:
    // 15 * (1 + 2 + 3 + 4 + 5 + ... + k)
    // or:
    // 15 * k * (k + 1) / 2
    // where k = (UPPER_LIMIT - 1) / 15
    // now we can solve the problem

    let n = (UPPER_LIMIT - 1) / 3;
    let m = (UPPER_LIMIT - 1) / 5;
    let k = (UPPER_LIMIT - 1) / 15;

    let sum_3 = 3 * n * (n + 1) / 2;
    let sum_5 = 5 * m * (m + 1) / 2;
    let sum_15 = 15 * k * (k + 1) / 2;

    (sum_3 + sum_5 - sum_15).to_string()
}


// this is the straight forward solution, but the above is better
//fn solve() -> String {
//    let mut sum: usize = 0;
//    for i in 1..1000 {
//        if (i % 3 == 0) || (i % 5 == 0) {
//            sum += i;
//        }
//    }
//    sum.to_string()
//}
