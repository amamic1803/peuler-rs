use crate::linalg::{Matrix, Point as linalgPoint, Vector};
use num_traits::{ConstOne, FromPrimitive, PrimInt, ToPrimitive};
use std::borrow::Borrow;

pub type Point<T, const N: usize> = linalgPoint<T, N>;

pub struct Polygon<T> {
    points: Vec<Point<T, 2>>,
}
impl<T: Copy + ToPrimitive> Polygon<T> {
    /// Create a new [Polygon].
    /// # Arguments
    /// * `points` - An iterable collection of points that define the polygon.
    /// # Returns
    /// * A new [Polygon] instance.
    pub fn new<U, V>(points: U) -> Self
    where
        U: IntoIterator<Item = V>,
        V: Borrow<Point<T, 2>>,
    {
        let points = points.into_iter().map(|p| *p.borrow()).collect();
        Self { points }
    }

    /// Calculate the area of the polygon.
    ///
    /// Uses the shoelace formula.
    /// # Returns
    /// * The area of the polygon.
    pub fn area(&self) -> f64 {
        let mut area = 0.0;
        for i in 0..self.points.len() {
            let p1 = self.points[i].coords();
            let p2 = self.points[(i + 1) % self.points.len()].coords();
            let matrix = Matrix::new([[p1[0], p2[0]], [p1[1], p2[1]]]);
            area += matrix.determinant();
        }
        (area / 2.0).abs()
    }

    /// Calculate the perimeter of the polygon.
    /// # Returns
    /// * The perimeter of the polygon.
    pub fn perimeter(&self) -> f64 {
        let mut perimeter = 0.0;
        for i in 0..self.points.len() {
            perimeter +=
                Vector::from_points(self.points[i], self.points[(i + 1) % self.points.len()])
                    .magnitude();
        }
        perimeter
    }
}
impl<T> Polygon<T>
where
    T: Copy + PrimInt + ConstOne + FromPrimitive,
{
    /// Calculate the number of boundary points of the polygon.
    ///
    /// Boundary points are points with integer coordinates that lie on the edges of the polygon.
    /// The polygon must be defined by points with integer coordinates.
    /// # Returns
    /// * The number of boundary points of the polygon.
    pub fn boundary_points(&self) -> T {
        let mut points_count = T::from_usize(self.points.len())
            .expect("The number of points must be convertible to T");

        for i in 0..self.points.len() {
            // one of the coordinates is the same, so we can just add the difference of both coordinates
            // the result will be the distance between the two points
            // since we already counted all edge points, we just need to subtract 1
            // to get the number of points between the two points

            let p1 = self.points[i].coords();
            let p2 = self.points[(i + 1) % self.points.len()].coords();
            let diff0 = if p1[0] > p2[0] {
                p1[0] - p2[0]
            } else {
                p2[0] - p1[0]
            };
            let diff1 = if p1[1] > p2[1] {
                p1[1] - p2[1]
            } else {
                p2[1] - p1[1]
            };
            points_count = points_count + diff0 + diff1 - T::ONE;
        }

        points_count
    }

    /// Calculate the number of interior points of the polygon.
    ///
    /// Interior points are points with integer coordinates that lie strictly inside the polygon.
    /// The polygon must be defined by points with integer coordinates.
    /// Uses the Pick's theorem.
    pub fn interior_points(&self) -> T {
        let area = self.area();
        let boundary_points = self.boundary_points();

        // area = i + b/2 - 1
        // i = interior points
        // b = boundary points
        // i = area - b/2 + 1

        T::from_f64(
            (area
                - (boundary_points
                    .to_f64()
                    .expect("The number of boundary points must be convertible to f64"))
                    / 2.0
                + 1.0)
                .round(),
        )
        .expect("The number of interior points must be convertible to T")
    }
}
