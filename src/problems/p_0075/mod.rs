//! **Problem 75** - *Singular Integer Right Triangles*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(75, "Singular Integer Right Triangles", solve)
}

use crate::shared::math::gcd;

// maximum perimeter
const L_MAX: u32 = 1_500_000;

fn solve() -> String {
    // vector that stores numbers of integer sided right-angled triangles with perimeter <= L_MAX
    // position in vector represents perimeter
    let mut array = vec![0; L_MAX as usize + 1];

    // we will generate primitive pythagorean triples using Euclid's formula
    // L = perimeter
    // L = a + b + c
    // a = m^2 - n^2
    // b = 2mn
    // c = m^2 + n^2
    // L = 2m(m + n)
    // where m, n are positive integers
    // and m > n > 0
    // and m and n are coprime and exactly one of them is even
    // gcd(m, n) = 1
    // m + n is odd

    // to get other, non-primitive triples, we just multiply each side (or the whole perimeter) by some integer k
    // that is when we find primitive triple with some perimeter, we mark that perimeter as having a triangle,
    // but we also mark all multiples of that perimeter as having a triangle

    // the upper limit on n is m
    // let's find upper limit on m
    // L = 2m(m + n)
    // 2m^2 < 2m(m + n) <= L_MAX
    // 2m^2 < L_MAX
    // m^2 < L_MAX / 2
    // m < sqrt(L_MAX / 2)

    // we take ceil here because the iterator will stop at m = upper_m - 1
    let upper_m = ((L_MAX / 2) as f64).sqrt().ceil() as u32;

    for m in 1..upper_m {
        for n in 1..m {
            let l = 2 * m * (m + n); // perimeter
            if l > L_MAX {
                break;
            } // if perimeter is too big, we can stop checking bigger n

            // if m and n are coprime and m + n is odd (primitive triple)
            if gcd(m, n) == 1 && (m + n) % 2 == 1 {
                // mark all multiples of perimeter as having a triangle
                let mut current_l = l;
                while current_l <= L_MAX {
                    array[current_l as usize] += 1;
                    current_l += l;
                }
            }
        }
    }

    // count number perimeters with only one triangle
    array.into_iter().filter(|&x| x == 1).count().to_string()
}
