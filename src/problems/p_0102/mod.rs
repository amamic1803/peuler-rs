//! **Problem 102** - *Triangle Containment*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(102, "Triangle Containment", solve)
}

use crate::shared::math::{Point2D, Vector2D};
use std::f64::consts::PI;

const FULL_ANGLE: f64 = 2.0 * PI;

fn solve() -> String {
    let input_str = include_str!("0102_triangles.txt");
    let triangles = parse_input(input_str);

    // for every triangle, generate vectors from origin to each point
    // calculate and sum the angles between each vector
    // if the sum is 2pi, the triangle contains the origin
    // if the point is outside the triangle, the sum will be less than 2pi
    // count the number of triangles that contain the origin

    let mut count = 0;

    for triangle in triangles {
        let mut angle_sum = 0.0;
        let vec1 = Vector2D::from_points(Point2D::new(0.0, 0.0), triangle.0);
        let vec2 = Vector2D::from_points(Point2D::new(0.0, 0.0), triangle.1);
        let vec3 = Vector2D::from_points(Point2D::new(0.0, 0.0), triangle.2);

        angle_sum += vec1.angle_between(&vec2);
        angle_sum += vec2.angle_between(&vec3);
        angle_sum += vec3.angle_between(&vec1);

        if (angle_sum - FULL_ANGLE).abs() < 1e-8 {
            count += 1;
        }
    }

    count.to_string()
}

fn parse_input(input: &str) -> Vec<(Point2D, Point2D, Point2D)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut line_iter = line
                .trim()
                .split(',')
                .map(|num_str| num_str.parse::<f64>().unwrap());
            (
                Point2D::new(line_iter.next().unwrap(), line_iter.next().unwrap()),
                Point2D::new(line_iter.next().unwrap(), line_iter.next().unwrap()),
                Point2D::new(line_iter.next().unwrap(), line_iter.next().unwrap()),
            )
        })
        .collect::<Vec<(Point2D, Point2D, Point2D)>>()
}
