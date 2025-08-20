use crate::Solution;
use pmath::gcd;
use std::cmp::min;

problem!(Problem0091, 91, "Right Triangles with Integer Coordinates");

impl Solution for Problem0091 {
    fn solve(&self) -> String {
        const GRID_SIZE: u32 = 50;

        // let GRID_SIZE be the size of the grid (x, y) where x, y <= GRID_SIZE
        // the coordinates of the origin are (0, 0)

        // this problem can be separated into 4 smaller parts:
        // 1. triangles with right angle at origin
        // 2. triangles with right angle at x-axis or y-axis
        // 3. triangles with right angle at (x, y) where x == y (diagonal)
        // 4. triangles with right angle at (x, y) where x != y (non-diagonal)

        // 1. is easy, two points need to be chosen, one on x-axis and one on y-axis
        // there are N points on each axis != 0,
        // so for each point on x-axis, there are N points on y-axis
        // therefore, there are N * N triangles with right angle at origin
        let n1 = GRID_SIZE * GRID_SIZE;

        // 2. is also easy, two points need to be chosen, one somewhere in the field, not on x-axis or y-axis
        // the other point lies on either x-axis or y-axis
        // it is obvious that for any point in the field, there is a point on x-axis and a point on y-axis
        // that forms a right triangle with the point in the field (right angle is at x-axis or y-axis)
        // there are a total of N * N points in the field (N is not the element of the axes)
        // because every point can form 2 right triangles, there are 2 * N * N triangles with right angle at x-axis or y-axis
        let n2 = 2 * GRID_SIZE * GRID_SIZE;

        // 3. is a little harder, two points need to be chosen
        // the first point is on the diagonal, so obviously there are GRID_SIZE points to choose from
        // it is fine to find the number of right triangles only below the diagonal since for every right triangle below the diagonal
        // there is a right triangle above the diagonal that is the same
        // therefore the total number of right triangles with right angle at diagonal 2 * (number of right triangles below the diagonal)
        // it is possible to write out the number of right triangles below the diagonal for each point on the diagonal, for smaller values of GRID_SIZE
        // for example, for GRID_SIZE = 5, the number of right triangles below the diagonal for each point on the diagonal is:
        // 0: 0
        // 1: 1
        // 2: 2
        // 3: 2
        // 4: 1
        // 5: 0
        // for GRID_SIZE = 6
        // 0: 0
        // 1: 1
        // 2: 2
        // 3: 3
        // 4: 2
        // 5: 1
        // 6: 0
        // for GRID_SIZE = 7
        // 0: 0
        // 1: 1
        // 2: 2
        // 3: 3
        // 4: 3
        // 5: 2
        // 6: 1
        // 7: 0
        // obviously, for odd values of GRID_SIZE, the number of right triangles below the diagonal is
        // symmetric and equal to (GRID_SIZE / 2) * ((GRID_SIZE / 2) + 1)
        // the number of right triangles when GRID_SIZE is even can be calculated by subtracting GRID_SIZE / 2
        // from the number of right triangles when GRID_SIZE is odd
        let mut n3 = (GRID_SIZE / 2) * ((GRID_SIZE / 2) + 1); // num of triangles when GRID_SIZE is odd
        if GRID_SIZE % 2 == 0 {
            n3 -= GRID_SIZE / 2; // subtract GRID_SIZE / 2 if GRID_SIZE is even
        }
        n3 *= 2; // multiply by 2 to get the total number of right triangles with right angle at diagonal (above and below)

        // 4. is the hardest, two points need to be chosen somewhere in the field
        // the first point can be chosen somewhere in the field, not on the diagonal
        // and the second point can be calculated from the first point
        // actually there is no need for calculating the second point, it will be shown that by knowing the first point
        // the number of right triangles with right angle at (x, y) where x != y can be calculated
        // it is also enough to check only points below the diagonal since for every right triangle below the diagonal
        // there is a symmetric right triangle above the diagonal that is the same
        // let (x, y) be the coordinates of the first point
        // y = 0 needs to be excluded since it is on the x-axis (already counted in 2.)
        // therefore the loop for y starts from 1
        // the loop for x starts from y + 1 since x != y
        // and x > y since the point needs to be below the diagonal
        // on side of the triangle is the line segment connecting the origin and the first point
        // the slope of this line segment is y / x
        // the slope of the normal line to this line segment is -x / y
        // only integer coordinates are allowed
        // this means that by moving y units to the right and x units down, the second point is reached
        // also by moving y units to the left and x units up, the second point is reached
        // but, there may be other integer coordinates points somewhere in between
        // these can be found by first reducing the slope to its lowest terms (using gcd)
        // let i = reduced x
        // let j = reduced y
        // now the limitation of how many integer coordinates points can be generated (and therefore how many right triangles)
        // is the size of the grid (GRID_SIZE)
        // let's first find the number of points/triangles by moving j units to the right and i units down
        // the first limitation is that the point needs to be above or at the x-axis
        // therefore it is possible to choose at most y / i points
        // the second limitation is that the point's y coordinate needs to be below or at the GRID_SIZE
        // therefore it is possible to choose at most (GRID_SIZE - x) / j points
        // so the total number of points/triangles is min(y / i, (GRID_SIZE - x) / j)
        // by applying the same logic for moving y units to the left and x units up, the total number of points/triangles is
        // min(x / j, (GRID_SIZE - y) / i)
        let mut n4 = 0;
        for y in 1..=GRID_SIZE {
            for x in (y + 1)..=GRID_SIZE {
                let gcd_val = gcd(x, y);
                let i = x / gcd_val;
                let j = y / gcd_val;
                n4 += min(y / i, (GRID_SIZE - x) / j); // moving right/down
                n4 += min(x / j, (GRID_SIZE - y) / i); // moving up/left
            }
        }
        n4 *= 2; // multiply by 2 to get the total number of right triangles (above and below diagonal)

        // at the end, sum the number of right triangles
        (n1 + n2 + n3 + n4).to_string()
    }
}
