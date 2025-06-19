//! Linear algebra structures and operations.

use itertools::izip;
use std::ops::{Add, Mul, Sub};

/// A point in an N-dimensional space.
/// # Example
/// ```
/// use peuler::math::linalg::Point;
/// let point = Point::new([1.0, 2.0, 3.0]);
/// assert_eq!(point.coords, [1.0, 2.0, 3.0]);
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point<const N: usize> {
    pub coords: [f64; N],
}
impl<const N: usize> Point<N> {
    /// Creates a new point.
    /// # Arguments
    /// * `coords` - The coordinates of the point.
    /// # Returns
    /// * `Point` - The new point.
    pub fn new(coords: [f64; N]) -> Self {
        Self { coords }
    }
}

/// A vector in N-dimensional space.
/// # Example
/// ```
/// use peuler::math::linalg::Vector;
///
/// let vector = Vector::new([1.0, 2.0, 3.0]);
/// assert_eq!(vector.coords, [1.0, 2.0, 3.0]);
///
/// let quad_vector = 2.0 * vector * 2.0;
/// assert_eq!(quad_vector.coords, [4.0, 8.0, 12.0]);
///
/// let vector2 = Vector::new([4.0, 5.0, 6.0]);
/// let add_vector = vector + vector2;
/// let sub_vector = vector - vector2;
/// assert_eq!(add_vector.coords, [5.0, 7.0, 9.0]);
/// assert_eq!(sub_vector.coords, [-3.0, -3.0, -3.0]);
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<const N: usize> {
    pub coords: [f64; N],
}
impl<const N: usize> Vector<N> {
    /// Creates a new vector.
    /// # Arguments
    /// * `coords` - The coordinates of the vector.
    /// # Returns
    /// * `Vector` - The vector.
    pub fn new(coords: [f64; N]) -> Self {
        Self { coords }
    }
    /// Creates a new vector from 2 points.
    /// # Arguments
    /// * `point1` - The starting point.
    /// * `point2` - The ending point.
    /// # Returns
    /// * `Vector` - The vector from point1 to point2.
    /// # Example
    /// ```
    /// use peuler::math::linalg::{Point, Vector};
    /// let point1 = Point::new([1.0, 2.0, 3.0]);
    /// let point2 = Point::new([4.0, 5.0, 6.0]);
    /// let vector = Vector::from_points(point1, point2);
    /// assert_eq!(vector.coords, [3.0, 3.0, 3.0]);
    /// ```
    pub fn from_points(point1: Point<N>, point2: Point<N>) -> Self {
        let mut coords = [0.0; N];
        izip!(
            coords.iter_mut(),
            point1.coords.iter(),
            point2.coords.iter()
        )
        .for_each(|(coord, x, y)| *coord = y - x);
        Self { coords }
    }
    /// Calculates the angle between two vectors.
    /// The angle is in radians.
    /// # Arguments
    /// * `other` - The other vector.
    /// # Returns
    /// * `f64` - The angle between the two vectors.
    /// # Panics
    /// If at least one of the vectors is zero.
    /// # Example
    /// ```
    /// use peuler::math::linalg::Vector;
    /// let vector1 = Vector::new([1.0, 0.0]);
    /// let vector2 = Vector::new([0.0, 1.0]);
    /// assert_eq!(vector1.angle_between(&vector2), std::f64::consts::FRAC_PI_2);
    /// ```
    pub fn angle_between(&self, other: &Self) -> f64 {
        (self.dot_product(other) / (self.magnitude() * other.magnitude())).acos()
    }
    /// Calculates the cross product of two vectors.
    /// Only defined for 3D vectors.
    /// # Arguments
    /// * `other` - The other vector.
    /// # Returns
    /// * `Vector` - The cross product of the two vectors.
    /// # Example
    /// ```
    /// use peuler::math::linalg::Vector;
    /// let vector1 = Vector::new([1.0, 0.0, 0.0]);
    /// let vector2 = Vector::new([0.0, 1.0, 0.0]);
    /// assert_eq!(vector1.cross_product(&vector2).coords, [0.0, 0.0, 1.0]);
    /// ```
    pub fn cross_product(&self, other: &Self) -> Self {
        assert_eq!(N, 3, "Cross product is only defined for 3D vectors.");
        let mut coords = [0.0; N];
        coords[0] = self.coords[1] * other.coords[2] - self.coords[2] * other.coords[1];
        coords[1] = self.coords[2] * other.coords[0] - self.coords[0] * other.coords[2];
        coords[2] = self.coords[0] * other.coords[1] - self.coords[1] * other.coords[0];
        Self { coords }
    }
    /// Calculates the dot product of two vectors.
    /// # Arguments
    /// * `other` - The other vector.
    /// # Returns
    /// * `f64` - The dot product of the two vectors.
    /// # Example
    /// ```
    /// use peuler::math::linalg::Vector;
    /// let vector1 = Vector::new([1.0, 2.0, 3.0]);
    /// let vector2 = Vector::new([4.0, 5.0, 6.0]);
    /// assert_eq!(vector1.dot_product(&vector2), 32.0);
    /// ```
    pub fn dot_product(&self, other: &Self) -> f64 {
        self.coords
            .iter()
            .zip(other.coords.iter())
            .map(|(x, y)| x * y)
            .sum()
    }
    /// Calculates the magnitude of the vector.
    /// # Returns
    /// * `f64` - The magnitude of the vector.
    /// # Example
    /// ```
    /// use peuler::math::linalg::Vector;
    /// let vector = Vector::new([1.0, 2.0, 3.0]);
    /// assert_eq!(vector.magnitude(), 3.7416573867739413);
    /// ```
    pub fn magnitude(&self) -> f64 {
        self.coords.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }
    /// Calculates the normalized vector.
    /// # Returns
    /// * `Vector` - The normalized vector.
    /// # Panics
    /// If the vector is zero.
    /// # Example
    /// ```
    /// use peuler::math::linalg::Vector;
    /// let vector = Vector::new([1.0, 2.0, 3.0]);
    /// assert_eq!(vector.normalize().coords, [0.2672612419124244, 0.5345224838248488, 0.8017837257372732]);
    /// ```
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        let mut coords = [0.0; N];
        coords
            .iter_mut()
            .zip(self.coords.iter())
            .for_each(|(coord, x)| *coord = x / magnitude);
        Self { coords }
    }
}
impl<const N: usize> Add<Self> for Vector<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut coords = [0.0; N];
        izip!(coords.iter_mut(), self.coords.iter(), other.coords.iter())
            .for_each(|(coord, x, y)| *coord = x + y);
        Self { coords }
    }
}
impl<const N: usize> Sub<Self> for Vector<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut coords = [0.0; N];
        izip!(coords.iter_mut(), self.coords.iter(), other.coords.iter())
            .for_each(|(coord, x, y)| *coord = x - y);
        Self { coords }
    }
}
impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut coords = self.coords;
        coords.iter_mut().for_each(|coord| *coord *= rhs);
        Self { coords }
    }
}
impl<const N: usize> Mul<Vector<N>> for f64 {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        rhs * self
    }
}
