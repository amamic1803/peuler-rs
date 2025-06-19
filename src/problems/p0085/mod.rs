//! **Problem 85** - *Counting Rectangles*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(85, "Counting Rectangles", solve)
}

const RECTANGLE_COUNT: u64 = 2_000_000;

fn solve() -> String {
    // let grid be the size m x n
    // the number of rectangles in that grid is given by the formula:
    // (m * (m + 1) * n * (n + 1)) / 4
    // explanation:
    // picking one point can be done in (m + 1) * (n + 1) ways
    // picking other point that is not vertically or horizontally aligned with the first point can be done in
    // ((m + 1)(n + 1) - n - m + 1) = (m * n) ways
    // that is because all points in the same row or column are unavailable,
    // but the first point is counted (subtracted) twice so we add 1
    // so the expression is this:
    // m * (m + 1) * n * (n + 1)
    // but this counts each rectangle 4 times:
    // because the same rectangle is defined by two diagonals we divide by 2:
    // m * (m + 1) * n * (n + 1) / 2
    // but that still counts the same rectangle twice because for one diagonal one may chose first point in two ways (a b or b a)
    // so the final expression is this:
    // (m * (m + 1) * n * (n + 1)) / 4
    // we want to find the grid with the closest number of rectangles to 2 million
    // so:
    // m * (m + 1) * n * (n + 1) = 8 * 10^6
    // now we can use brute force to find the closest grid
    // the minimum value that m and n can take is 1
    // the maximum value can be found by setting the other variable to 1 and solving the equation
    // 1 * (1 + 1) * n * (n + 1) = 8 * 10^6
    // n * (n + 1) = 4 * 10^6
    // n^2 + n - 4 * 10^6 = 0
    // n = floor((-1 + sqrt(1 + 16 * 10^6)) / 2
    // now we can iterate m from 1 to that (maximum) value,
    // then calculate the value of n for that point and take the floor and ceil of that value
    // that gives us two combinations of m and n for each n
    // calculating n from m can be done by solving the equation:
    // n = (-1 + sqrt(1 + (32 * 10^6) / (m * (m + 1)))) / 2

    // RECTANGLE_COUNT constant is used so that the code can be easily modified to solve for other values
    // therefore the expressions are a bit more complicated than they need to be

    let max_val =
        ((-1.0 + (1.0 + 4.0 * ((4 * RECTANGLE_COUNT) as f64) / 2.0).sqrt()) / 2.0).floor() as u64;

    let mut closest_product = 0;
    let mut closest_diff = u64::MAX;
    for m in 1..=max_val {
        let n_exact = (-1.0
            + (1.0 + (4.0 * ((4 * RECTANGLE_COUNT) as f64)) / (m * (m + 1)) as f64).sqrt())
            / 2.0;
        let n_low = n_exact.floor() as u64;
        let n_high = n_exact.ceil() as u64;

        let rectangles_low = (m * (m + 1) * n_low * (n_low + 1)) / 4;
        let rectangles_low_diff = RECTANGLE_COUNT.abs_diff(rectangles_low);
        if rectangles_low_diff < closest_diff {
            closest_diff = rectangles_low_diff;
            closest_product = m * n_low;
        }

        let rectangles_high = (m * (m + 1) * n_high * (n_high + 1)) / 4;
        let rectangles_high_diff = RECTANGLE_COUNT.abs_diff(rectangles_high);
        if rectangles_high_diff < closest_diff {
            closest_diff = rectangles_high_diff;
            closest_product = m * n_high;
        }
    }

    closest_product.to_string()
}
