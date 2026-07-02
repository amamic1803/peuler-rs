use crate::Solution;
use pmath::geometry::Point;
use pmath::geometry::dim2::Triangle;

problem!(Problem0102, 102, "Triangle Containment");

impl Solution for Problem0102 {
    fn solve(&self) -> String {
        const INPUT: &str = include_str!("0102_triangles.txt");
        let origin = Point::new([0.0; 2]);
        let count = INPUT
            .trim()
            .lines()
            .filter(|line| {
                let mut line_iter = line
                    .trim()
                    .split(',')
                    .map(|num_str| num_str.parse::<f64>().unwrap());
                let triangle = Triangle::new([
                    Point::new([line_iter.next().unwrap(), line_iter.next().unwrap()]),
                    Point::new([line_iter.next().unwrap(), line_iter.next().unwrap()]),
                    Point::new([line_iter.next().unwrap(), line_iter.next().unwrap()]),
                ]);
                triangle.contains(&origin)
            })
            .count();

        count.to_string()
    }
}
