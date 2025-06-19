//! **Problem 73** - *Counting Fractions in a Range*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(73, "Counting Fractions in a Range", solve)
}

const MAX_DEN: u16 = 12_000; // maximum denominator

fn solve() -> String {
    // this is Farey sequence
    // for every pair of fractions a/b and c/d, we can find some fraction between them using the formula:
    // (a+c)/(b+d)
    // if b+d <= 12_000, we count the number of fractions between a/b and (a+c)/(b+d) and (a+c)/(b+d) and c/d, and add 1
    // this is recursive process, so we just continue until b+d > 12_000 in which case we return 0

    frac_between(1, 3, 1, 2).to_string()
}

/// Count the number of reduced proper fractions between `a/b` and `c/d` whose denominator does not exceed `MAX_DEN`.
fn frac_between(a: u16, b: u16, c: u16, d: u16) -> u32 {
    let new_den = b + d;

    if new_den > MAX_DEN {
        0
    } else {
        let new_num = a + c;
        1 + frac_between(a, b, new_num, new_den) + frac_between(new_num, new_den, c, d)
    }
}
