//! **Problem 15** - *Lattice Paths*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        15,
        "Lattice Paths",
        solve,
    )
}


fn solve() -> String {
    // there are 40 steps in total, 20 of which are right and 20 of which are down
    // we can think of this as a permutation with repetition problem (20 R's, 20 D's)
    // we get the number of permutations with repetition by doing 40! / (20! * 20!)
    // that is because 40! is the total number of permutations of 40 steps
    // but this includes permutations that are the same (because if two R's are swapped, it's still the same path)
    // so we have to divide by the amount of permutations of 20 R's and 20 D's (20! * 20!)

    // I simplified the expression by hand to avoid using a big number library
    // 40! / (20! * 20!) = 37 * 31 * 29 * 23 * 13 * 11 * 7 * 5 * 3 * 3 * 2 * 2

    let result: u64 = 37 * 31 * 29 * 23 * 13 * 11 * 7 * 5 * 3 * 3 * 2 * 2;
    result.to_string()
}
