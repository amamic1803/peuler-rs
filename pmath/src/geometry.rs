//! Geometry module for 2D and 3D shapes and their properties.

use crate::algebra::linear::{Matrix, Vector};
use num_traits::{ConstOne, FromPrimitive, PrimInt, ToPrimitive};
use std::borrow::Borrow;
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

/// A point in an N-dimensional space.
///
/// All overloaded operators are element-wise.
///
/// Coordinates can be accessed directly because [Deref] and [DerefMut] traits are implemented.
/// # Example
/// ```
/// use pmath::geometry::Point;
///
/// let mut point = Point::new([1, 2, 3]);
/// assert_eq!(*point, [1, 2, 3]);
///
/// *point = [4, 5, 6];
/// assert_eq!(*point, [4, 5, 6]);
/// ```
#[derive(Copy, Clone, PartialEq)]
pub struct Point<T, const N: usize> {
    coords: [T; N],
}
impl<T, const N: usize> Point<T, N> {
    /// Create a new [Point].
    /// # Arguments
    /// * `coords` - The coordinates of the point.
    /// # Returns
    /// * The new point.
    /// # Panics
    /// * If `N` is zero.
    pub fn new(coords: [T; N]) -> Self {
        if N == 0 {
            panic!("The number of dimensions must be greater than zero");
        }
        Self { coords }
    }
}
impl<T, const N: usize> Deref for Point<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}
impl<T, const N: usize> DerefMut for Point<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.coords
    }
}
impl<T, const N: usize> Add<Self> for Point<T, N>
where
    T: for<'a> AddAssign<&'a T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut new_point = self;
        new_point += other;
        new_point
    }
}
impl<T, const N: usize> AddAssign<Self> for Point<T, N>
where
    T: for<'a> AddAssign<&'a T>,
{
    fn add_assign(&mut self, other: Self) {
        self.coords
            .iter_mut()
            .zip(other.coords.iter())
            .for_each(|(coord, y)| *coord += y);
    }
}
impl<T, const N: usize> Sub<Self> for Point<T, N>
where
    T: for<'a> SubAssign<&'a T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut new_point = self;
        new_point -= other;
        new_point
    }
}
impl<T, const N: usize> SubAssign<Self> for Point<T, N>
where
    T: for<'a> SubAssign<&'a T>,
{
    fn sub_assign(&mut self, other: Self) {
        self.coords
            .iter_mut()
            .zip(other.coords.iter())
            .for_each(|(coord, y)| *coord -= y);
    }
}
impl<T, const N: usize> Mul<Self> for Point<T, N>
where
    T: for<'a> MulAssign<&'a T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_point = self;
        new_point *= rhs;
        new_point
    }
}
impl<T, const N: usize> MulAssign<Self> for Point<T, N>
where
    T: for<'a> MulAssign<&'a T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.coords
            .iter_mut()
            .zip(rhs.coords.iter())
            .for_each(|(coord, y)| *coord *= y);
    }
}
impl<T, const N: usize> Div<Self> for Point<T, N>
where
    T: for<'a> DivAssign<&'a T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut new_point = self;
        new_point /= rhs;
        new_point
    }
}
impl<T, const N: usize> DivAssign<Self> for Point<T, N>
where
    T: for<'a> DivAssign<&'a T>,
{
    fn div_assign(&mut self, rhs: Self) {
        self.coords
            .iter_mut()
            .zip(rhs.coords.iter())
            .for_each(|(coord, y)| *coord /= y);
    }
}
impl<T, const N: usize> Neg for Point<T, N>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for i in 0..self.coords.len() {
            self.coords[i] = -self.coords[i];
        }
        self
    }
}
impl<T, const N: usize> Mul<T> for Point<T, N>
where
    T: for<'a> MulAssign<&'a T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut new_point = self;
        new_point *= rhs;
        new_point
    }
}
impl<T, const N: usize> MulAssign<T> for Point<T, N>
where
    T: for<'a> MulAssign<&'a T>,
{
    fn mul_assign(&mut self, rhs: T) {
        self.coords.iter_mut().for_each(|coord| *coord *= &rhs);
    }
}
impl<T, const N: usize> Div<T> for Point<T, N>
where
    T: for<'a> DivAssign<&'a T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut new_point = self;
        new_point /= rhs;
        new_point
    }
}
impl<T, const N: usize> DivAssign<T> for Point<T, N>
where
    T: for<'a> DivAssign<&'a T>,
{
    fn div_assign(&mut self, rhs: T) {
        self.coords.iter_mut().for_each(|coord| *coord /= &rhs);
    }
}
macro_rules! impl_mul_scalar_by_point {
    ($($scalar:ty),*) => {
        $(
            impl<const N: usize> Mul<Point<$scalar, N>> for $scalar {
                type Output = Point<$scalar, N>;
                fn mul(self, rhs: Point<$scalar, N>) -> Self::Output {
                    rhs * self
                }
            }
        )*
    };
}
impl_mul_scalar_by_point!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

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

pub trait Shape3D {
    /// Calculate the surface area of the shape.
    /// # Returns
    /// * The surface area of the shape.
    fn surface_area(&self) -> f64;

    /// Calculate the volume of the shape.
    /// # Returns
    /// * The volume of the shape.
    fn volume(&self) -> f64;
}

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
            let p1 = self.points[i];
            let p2 = self.points[(i + 1) % self.points.len()];
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
