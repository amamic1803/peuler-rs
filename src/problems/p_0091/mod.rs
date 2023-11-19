//! **Problem 91** - *Right Triangles with Integer Coordinates*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        91,
        "Right Triangles with Integer Coordinates",
        solve,
    )
}


use std::cmp::min;

const GRID_SIZE: u32 = 50;

fn solve() -> String {

    let n1 = GRID_SIZE * GRID_SIZE;

    let n2 = 2 * GRID_SIZE * GRID_SIZE;

    let mut n3 = (GRID_SIZE / 2) * ((GRID_SIZE / 2) + 1);
    if GRID_SIZE % 2 == 0 {
        n3 -= GRID_SIZE / 2;
    }
    n3 *= 2;

    let mut n4 = 0;
    for y in 1..=((GRID_SIZE - 1) / 2) {
        for x in (y + 1)..=(GRID_SIZE - y) {
            n4 += min(x / y, (GRID_SIZE - y) / x);
        }
    }
    n4 *= 2;

    (n1 + n2 + n3 + n4).to_string()
}
