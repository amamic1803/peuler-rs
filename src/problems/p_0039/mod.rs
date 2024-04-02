//! **Problem 39** - *Integer Right Triangles*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(39, "Integer Right Triangles", solve)
}

use crate::shared::math::gcd;
use itertools::Itertools;

const MAX_P: u64 = 1000;

fn solve() -> String {
    // p = a + b + c

    // Euclid's formula:
    // let (m, n) be positive integers with following properties:
    // m > n
    // m and n are coprime
    // exactly one of m and n is even
    // then:
    // a = m^2 - n^2
    // b = 2mn
    // c = m^2 + n^2
    // and (a, b, c) is a primitive Pythagorean triple

    // let k be a positive integer
    // then (ka, kb, kc) is also a Pythagorean triple, but it is not primitive
    // (it can be reduced by k)

    // primary goal towards solving this problem is to generate primitive triples
    // and from them, generate their multiples

    // the number of triples with sum p will be stored in solutions vector
    // index 0 is unused
    let mut solutions = vec![0; MAX_P as usize + 1];

    // since m > n, we need to find the maximum m for given MAX_P
    // p = a + b + c = 2m(m + n)
    // p / 2 = m(m + n)
    // obviously, m will be maximum when n = 1, so:
    // p / 2 = m(m + 1)
    // m^2 + m = p / 2
    // m^2 + m = MAX_P / 2
    // m^2 = MAX_P / 2 - m
    // m = sqrt(MAX_P / 2 - m) < sqrt(MAX_P / 2)
    // so as the upper bound for m, we can take floor(sqrt(MAX_P / 2))
    // also, since m and n are positive integers, m > n, lower bound for m is 2
    let m_lower_bound = 2;
    let m_upper_bound = (MAX_P as f64 / 2.0).sqrt().floor() as u64;

    // for each m, we need to check every n in the range 1..m (exclusive)
    // for m and n to be a valid pair, they need to be coprime
    // so we need to check if gcd(m, n) == 1
    // also exactly one of m and n is even
    // we can check that by checking if m - n is odd
    // so we need to check if (m - n) % 2 == 1

    // now that m and n are a valid pair, we can calculate a primitive triple (a, b, c)
    // and p = a + b + c = 2m(m + n)
    // we don't actually care about a + b + c, so we will only calculate p

    // if we get p > MAX_P, we can stop checking n, since it will only get bigger, and move on to next m
    // if we get p <= MAX_P, we increment solutions[p] by 1
    // and also increment all solutions[kp] by 1, where k is a positive integer, and kp <= MAX_P

    for m in m_lower_bound..=m_upper_bound {
        for n in 1..m {
            if gcd(m, n) == 1 && (m - n) % 2 == 1 {
                let p = 2 * m * (m + n);
                if p > MAX_P {
                    break;
                }
                let mut k = p;
                while k <= MAX_P {
                    solutions[k as usize] += 1;
                    k += p;
                }
            }
        }
    }

    solutions.iter().position_max().unwrap().to_string()
}
