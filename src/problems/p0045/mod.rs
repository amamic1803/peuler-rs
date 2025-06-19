//! **Problem 45** - *Triangular, Pentagonal, and Hexagonal*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(45, "Triangular, Pentagonal, and Hexagonal", solve)
}

fn solve() -> String {
    // we can think of T(n), P(n) and H(n) as sequences of natural numbers

    // every triangular number is also hexagonal
    // proof:
    // T(n) = n(n+1)/2
    // H(m) = m(2m-1)
    // T(n) = H(m)
    // n(n+1)/2 = m(2m-1)
    // n(n+1) = 2m(2m-1)
    // n(n+1) =(2m-1)2m
    // from there we can read that
    // n = 2m-1
    // therefore for every hexagonal number at index m there is equal triangular number at index n=2m-1
    // so we can say that H(m) = T(2m-1)

    // now we know that if a number is hexagonal then
    // it is also triangular so we don't have to check if it is triangular

    // let n be a natural number bigger than 1
    // it can be proven (by induction) that P(n) < H(n) for all n > 1
    // that means that we can generate hexagonal numbers and check if they are pentagonal
    // which is more efficient than generating pentagonal numbers and checking if they are hexagonal

    // problem says that we are searching for number bigger than H(143) so we can start from 144
    let mut n = 144;
    let mut hex_num = n * (2 * n - 1);

    // each iteration we increase n by 1 and calculate next hexagonal number
    while !is_pentagonal(hex_num) {
        n += 1;
        hex_num = n * ((n << 1) - 1); // n * (2n - 1)
    }

    hex_num.to_string()
}

fn is_pentagonal(num: u64) -> bool {
    // when we know Pn (num) we can calculate n by solving the quadratic equation
    // 3n^2 - n - 2Pn = 0
    // n = (1 + sqrt(1 + 24Pn)) / 6
    // (as Pn is equal to or bigger than 1, we know that the square root is bigger than or equal to 5,
    // therefore the other solution would be negative so we can ignore it because n is a natural number)
    let i = (1.0 + ((1 + 24 * num) as f64).sqrt()) / 6.0;

    (i.round() - i).abs() <= 1e-10
}
