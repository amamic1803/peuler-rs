//! A module for 2D geometry.

use crate::algebra::linear::{Matrix, Vector};
use crate::core::gcd;
use crate::geometry::Point;
use num_traits::{ConstOne, ConstZero, FromPrimitive, PrimInt, ToPrimitive};
use std::f64::consts::PI;
use std::ops::{Deref, DerefMut};

/// A 2D shape.
pub trait Shape2D {
    /// Calculate the area of the shape.
    /// # Returns
    /// * The area of the shape.
    fn area(&self) -> f64;

    /// Calculate the perimeter of the shape.
    /// # Returns
    /// * The perimeter of the shape.
    fn perimeter(&self) -> f64;
}

/// A polygon in 2D space.
///
/// The polygon is defined by `N` points, vertices.
/// Points can be accessed directly since [Polygon] implements [Deref] and [DerefMut] traits.
/// # Example
/// ```
/// use pmath::geometry::Point;
/// use pmath::geometry::dim2::{Polygon, Shape2D};
///
/// let points = [Point::new([0, 0]), Point::new([4, 0]), Point::new([4, 3]), Point::new([0, 3])];
/// let polygon = Polygon::new(points);
///
/// assert!((polygon.area() - 12.0).abs() < 1e-10);
/// assert!((polygon.perimeter() - 14.0).abs() < 1e-10);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Polygon<T, const N: usize> {
    points: [Point<T, 2>; N],
}
impl<T, const N: usize> Polygon<T, N> {
    /// Create a new [Polygon].
    /// # Arguments
    /// * `points` - The points of the polygon. The order matters since it defines the edges of the polygon.
    /// # Returns
    /// * A new [Polygon] instance.
    pub fn new(points: [Point<T, 2>; N]) -> Self {
        Self { points }
    }

    /// The number of boundary points of the polygon.
    ///
    /// Boundary points are points with integer coordinates that lie on the edges of the polygon.
    /// The polygon must be defined by points with integer coordinates.
    /// # Returns
    /// * The number of boundary points of the polygon.
    pub fn boundary_points(&self) -> T
    where
        T: PrimInt + FromPrimitive + ConstOne + ConstZero,
    {
        let mut points_count = T::from_usize(self.points.len())
            .expect("The number of points must be convertible to T");

        for i in 0..self.points.len() {
            let p1 = self.points[i];
            let p2 = self.points[(i + 1) % self.points.len()];
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
            points_count = points_count + gcd(diff0, diff1) - T::ONE;
        }

        points_count
    }

    /// The number of interior points of the polygon.
    ///
    /// Interior points are points with integer coordinates that lie strictly inside the polygon.
    /// The polygon must be defined by points with integer coordinates.
    /// Uses the *Pick's theorem* and therefore only works correctly for
    /// simple polygons (without holes or edge intersections).
    /// # Returns
    /// * The number of interior points of the polygon.
    pub fn interior_points(&self) -> T
    where
        T: Copy + FromPrimitive + ConstZero + ConstOne + PrimInt,
    {
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
impl<T, const N: usize> Shape2D for Polygon<T, N>
where
    T: Copy + ToPrimitive,
{
    fn area(&self) -> f64 {
        // shoelace formula
        let mut area = 0.0;
        for i in 0..self.points.len() {
            let p1 = self.points[i];
            let p2 = self.points[(i + 1) % self.points.len()];
            let matrix = Matrix::new([[p1[0], p2[0]], [p1[1], p2[1]]]);
            area += matrix.determinant();
        }
        (area / 2.0).abs()
    }

    fn perimeter(&self) -> f64 {
        let mut perimeter = 0.0;
        for i in 0..self.points.len() {
            perimeter +=
                Vector::from_points(self.points[i], self.points[(i + 1) % self.points.len()])
                    .magnitude();
        }
        perimeter
    }
}
impl<T, const N: usize> Deref for Polygon<T, N> {
    type Target = [Point<T, 2>; N];

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}
impl<T, const N: usize> DerefMut for Polygon<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.points
    }
}

/// A triangle in 2D space.
/// # Example
/// ```
/// use pmath::geometry::Point;
/// use pmath::geometry::dim2::{Triangle, Shape2D};
///
/// let points = [Point::new([0, 0]), Point::new([8, 0]), Point::new([4, 3])];
/// let triangle = Triangle::new(points);
///
/// assert!((triangle.area() - 12.0).abs() < 1e-10);
/// assert!((triangle.perimeter() - 18.0).abs() < 1e-10);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Triangle<T> {
    polygon: Polygon<T, 3>,
}
impl<T> Triangle<T> {
    /// Create a new [Triangle].
    /// # Arguments
    /// * `points` - The points/vertices of the triangle.
    /// # Returns
    /// * A new [Triangle] instance.
    pub fn new(points: [Point<T, 2>; 3]) -> Self {
        Self {
            polygon: Polygon::new(points),
        }
    }
    /// Check if the triangle contains a point.
    ///
    /// The triangle contains a point if it is located
    /// inside the triangle. Points located on the edges
    /// might be classified either as contained or not contained
    /// in the triangle (because of rounding error).
    /// # Arguments
    /// * `point` - The point to check.
    /// # Returns
    /// * `true` if the triangle contains the point, `false` otherwise.
    pub fn contains<U>(&self, point: &Point<U, 2>) -> bool
    where
        T: ToPrimitive + Copy,
        U: ToPrimitive + Copy,
    {
        // generate vectors from the point to each vertex of the triangle
        // calculate and sum the angles between each vector
        // if the sum is 2pi, the triangle contains the point
        // if the point is outside the triangle, the sum will be less than 2pi

        let mut angle_sum = 0.0;

        let vec1 = Vector::from_points(*point, self.polygon[0]);
        let vec2 = Vector::from_points(*point, self.polygon[1]);
        let vec3 = Vector::from_points(*point, self.polygon[2]);

        angle_sum += vec1.angle_between(&vec2);
        angle_sum += vec2.angle_between(&vec3);
        angle_sum += vec3.angle_between(&vec1);

        (angle_sum - 2.0 * PI).abs() < 1e-10
    }
    /// Get reference to the vertices of the triangle.
    /// # Returns
    /// * A reference to the vertices of the triangle.
    pub fn get_points(&self) -> &[Point<T, 2>; 3] {
        &self.polygon
    }
    /// Get mutable reference to the vertices of the triangle
    /// # Returns
    /// * A mutable reference to the vertices of the triangle.
    pub fn get_points_mut(&mut self) -> &mut [Point<T, 2>; 3] {
        &mut self.polygon
    }
}
impl<T> Shape2D for Triangle<T>
where
    T: Copy + ToPrimitive,
{
    fn area(&self) -> f64 {
        self.polygon.area()
    }

    fn perimeter(&self) -> f64 {
        self.polygon.perimeter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::assert_float_absolute_eq;

    // polygon

    #[test]
    fn polygon_new() {
        //! Test that [Polygon::new] works.

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([1, 1]),
            Point::new([0, 1]),
        ];
        let polygon = Polygon::new(points);
        assert_eq!(polygon.points, points);
    }

    #[test]
    fn polygon_deref() {
        //! Test that [Deref] and [DerefMut] implementations work on [Polygon].

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([1, 1]),
            Point::new([0, 1]),
        ];
        let mut polygon = Polygon::new(points);
        assert_eq!(*polygon, points);

        let points2 = [
            Point::new([0, 0]),
            Point::new([2, 0]),
            Point::new([2, 2]),
            Point::new([0, 2]),
        ];
        *polygon = points2;
        assert_eq!(*polygon, points2);
    }

    #[test]
    fn polygon_boundary_points() {
        //! Verify that [Polygon::boundary_points] works correctly.

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([1, 1]),
            Point::new([0, 1]),
        ];
        assert_eq!(Polygon::new(points).boundary_points(), 4);

        let points = [
            Point::new([0, 0]),
            Point::new([2, 0]),
            Point::new([2, 2]),
            Point::new([0, 2]),
        ];
        assert_eq!(Polygon::new(points).boundary_points(), 8);

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([2, 2]),
            Point::new([1, 2]),
        ];
        assert_eq!(Polygon::new(points).boundary_points(), 4);

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([3, 4]),
            Point::new([2, 4]),
        ];
        assert_eq!(Polygon::new(points).boundary_points(), 6);

        let points = [
            Point::new([2, 2]),
            Point::new([1, 0]),
            Point::new([-4, 2]),
            Point::new([-2, -2]),
        ];
        assert_eq!(Polygon::new(points).boundary_points(), 8);
    }

    #[test]
    fn polygon_interior_points() {
        //! Verify the correctness of [Polygon::interior_points].
        //! Tests only simple polygons as required by the function.

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([1, 1]),
            Point::new([0, 1]),
        ];
        assert_eq!(Polygon::new(points).interior_points(), 0);

        let points = [
            Point::new([0, 0]),
            Point::new([2, 0]),
            Point::new([2, 2]),
            Point::new([0, 2]),
        ];
        assert_eq!(Polygon::new(points).interior_points(), 1);

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([2, 2]),
            Point::new([1, 2]),
        ];
        assert_eq!(Polygon::new(points).interior_points(), 1);

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([3, 4]),
            Point::new([2, 4]),
        ];
        assert_eq!(Polygon::new(points).interior_points(), 2);

        let points = [
            Point::new([1, -2]),
            Point::new([1, 0]),
            Point::new([-4, 2]),
            Point::new([-2, -2]),
        ];
        assert_eq!(Polygon::new(points).interior_points(), 8);
    }

    #[test]
    fn polygon_perimeter() {
        //! Verify the correctness of [Polygon::perimeter].
        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([1, 1]),
            Point::new([0, 1]),
        ];
        assert_float_absolute_eq!(Polygon::new(points).perimeter(), 4.0);

        let points = [
            Point::new([0, 0]),
            Point::new([2, 0]),
            Point::new([2, 2]),
            Point::new([0, 2]),
        ];
        assert_float_absolute_eq!(Polygon::new(points).perimeter(), 8.0);

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([2, 2]),
            Point::new([1, 2]),
        ];
        assert_float_absolute_eq!(Polygon::new(points).perimeter(), 2.0 + 2.0 * 5.0_f64.sqrt());

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([3, 4]),
            Point::new([2, 4]),
        ];
        assert_float_absolute_eq!(
            Polygon::new(points).perimeter(),
            2.0 + 2.0 * 20.0_f64.sqrt()
        );

        let points = [
            Point::new([2, 2]),
            Point::new([1, 0]),
            Point::new([-4, 2]),
            Point::new([-2, -2]),
        ];
        assert_float_absolute_eq!(
            Polygon::new(points).perimeter(),
            5.0f64.sqrt() + 29.0f64.sqrt() + 20.0f64.sqrt() + 32.0f64.sqrt()
        );
    }

    #[test]
    fn polygon_area() {
        //! Verify the correctness of [Polygon::area].

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([1, 1]),
            Point::new([0, 1]),
        ];
        assert_float_absolute_eq!(Polygon::new(points).area(), 1.0);

        let points = [
            Point::new([0, 0]),
            Point::new([2, 0]),
            Point::new([2, 2]),
            Point::new([0, 2]),
        ];
        assert_float_absolute_eq!(Polygon::new(points).area(), 4.0);

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([2, 2]),
            Point::new([1, 2]),
        ];
        assert_float_absolute_eq!(Polygon::new(points).area(), 2.0);

        let points = [
            Point::new([0, 0]),
            Point::new([1, 0]),
            Point::new([3, 4]),
            Point::new([2, 4]),
        ];
        assert_float_absolute_eq!(Polygon::new(points).area(), 4.0);

        let points = [
            Point::new([2, 2]),
            Point::new([1, 0]),
            Point::new([-4, 2]),
            Point::new([-2, -2]),
        ];
        assert_float_absolute_eq!(Polygon::new(points).area(), 6.0);
    }

    // triangle

    #[test]
    fn triangle_new() {
        //! Test that [Triangle::new] works.

        let points = [Point::new([0, 0]), Point::new([1, 0]), Point::new([1, 1])];
        let triangle = Triangle::new(points);
        assert_float_absolute_eq!(triangle.area(), 0.5);
    }

    #[test]
    fn triangle_contains() {
        //! Test that [Triangle::contains] works correctly.

        let points = [Point::new([0, 0]), Point::new([1, 0]), Point::new([1, 1])];
        let triangle = Triangle::new(points);
        assert!(triangle.contains(&Point::new([0.52, 0.5])));
        assert!(triangle.contains(&Point::new([1.0, 0.99])));
        assert!(triangle.contains(&Point::new([0.5, 0.001])));
        assert!(!triangle.contains(&Point::new([0.5, -0.001])));
        assert!(!triangle.contains(&Point::new([2, 2])));
    }

    #[test]
    fn triangle_get_points() {
        //! Test [Triangle::get_points] and [Triangle::get_points_mut] methods.

        let points = [Point::new([0, 0]), Point::new([1, 0]), Point::new([1, 1])];
        let mut triangle = Triangle::new(points);
        assert_eq!(triangle.get_points(), &points);
        triangle.get_points_mut()[1] = Point::new([0, 1]);
        assert_eq!(triangle.get_points()[1], Point::new([0, 1]));
    }

    #[test]
    fn triangle_perimeter() {
        //! Test that [Triangle::perimeter] works correctly.

        let points = [Point::new([0, 0]), Point::new([1, 0]), Point::new([1, 1])];
        assert_float_absolute_eq!(Polygon::new(points).perimeter(), 2.0 + 2.0f64.sqrt());

        let points = [
            Point::new([0.0, 0.0]),
            Point::new([3.0, 0.0]),
            Point::new([1.5, 5.0]),
        ];
        assert_float_absolute_eq!(
            Polygon::new(points).perimeter(),
            3.0 + 2.0 * 27.25f64.sqrt()
        );
    }

    #[test]
    fn triangle_area() {
        //! Test that [Triangle::area] works correctly.

        let points = [Point::new([0, 0]), Point::new([1, 0]), Point::new([1, 1])];
        assert_float_absolute_eq!(Polygon::new(points).area(), 0.5);

        let points = [
            Point::new([0.0, 0.0]),
            Point::new([3.0, 0.0]),
            Point::new([1.5, 5.0]),
        ];
        assert_float_absolute_eq!(Polygon::new(points).area(), 7.5);

        let points = [
            Point::new([3.741204046097, 0.9302469514332]),
            Point::new([2.1958323446875, -0.6644451234254]),
            Point::new([2.1629520957214, 0.8562663912594]),
        ];
        assert_float_absolute_eq!(Polygon::new(points).area(), 1.201249207);
    }
}
