//! Linear algebra structures and operations.

use std::ops::{Add, Mul, Sub};
use itertools::izip;
use num_traits::ToPrimitive;

/// A point in an N-dimensional space.
/// # Example
/// ```
/// use peuler::math::linalg::Point;
///
/// let mut point = Point::new([1, 2, 3]);
/// assert_eq!(*point.coords(), [1.0, 2.0, 3.0]);
///
/// point.set_coords([4, 5, 6]);
/// assert_eq!(*point.coords(), [4.0, 5.0, 6.0]);
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point<const N: usize> {
    coords: [f64; N],
}
impl<const N: usize> Point<N> {
    /// Creates a new point.
    /// # Arguments
    /// * `coords` - The coordinates of the point.
    /// # Returns
    /// * `Point` - The new point.
    /// # Panics
    /// * If the coordinates cannot be converted to [f64].
    pub fn new<T: ToPrimitive>(coords: [T; N]) -> Self {
        let coords = coords.map(|x| x.to_f64().expect("Coordinates must be convertible to f64."));
        Self { coords }
    }
    /// Gets the coordinates of the point.
    /// # Returns
    /// * `&[f64; N]` - The coordinates of the point.
    pub fn coords(&self) -> &[f64; N] {
        &self.coords
    }
    /// Gets a mutable reference to the coordinates of the point.
    /// # Returns
    /// * `&mut [f64; N]` - A mutable reference to the coordinates of the point.
    pub fn coords_mut(&mut self) -> &mut [f64; N] {
        &mut self.coords
    }
    /// Sets the coordinates of the point.
    /// # Arguments
    /// * `coords` - The new coordinates of the point.
    /// # Panics
    /// * If the coordinates cannot be converted to [f64].
    pub fn set_coords<T: ToPrimitive>(&mut self, coords: [T; N]) {
        self.coords = coords.map(|x| x.to_f64().expect("Coordinates must be convertible to f64."));
    }
}

/// A vector in an N-dimensional space.
/// # Example
/// ```
/// use peuler::math::linalg::Vector;
///
/// let vector = Vector::new([1, 2, 3]);
/// assert_eq!(*vector.coords(), [1.0, 2.0, 3.0]);
///
/// let quad_vector = vector * 4;
/// assert_eq!(*quad_vector.coords(), [4.0, 8.0, 12.0]);
///
/// let vector2 = Vector::new([4, 5, 6]);
/// let add_vector = vector + vector2;
/// let sub_vector = vector - vector2;
/// assert_eq!(*add_vector.coords(), [5.0, 7.0, 9.0]);
/// assert_eq!(*sub_vector.coords(), [-3.0, -3.0, -3.0]);
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<const N: usize> {
    coords: [f64; N],
}
impl<const N: usize> Vector<N> {
    /// Creates a new vector.
    /// # Arguments
    /// * `coords` - The coordinates of the vector.
    /// # Returns
    /// * `Vector` - The vector.
    /// # Panics
    /// * If the coordinates cannot be converted to [f64].
    pub fn new<T: ToPrimitive>(coords: [T; N]) -> Self {
        let coords = coords.map(|x| x.to_f64().expect("Coordinates must be convertible to f64."));
        Self { coords }
    }
    /// Gets the coordinates of the vector.
    /// # Returns
    /// * `&[f64; N]` - The coordinates of the vector.
    pub fn coords(&self) -> &[f64; N] {
        &self.coords
    }
    /// Gets a mutable reference to the coordinates of the vector.
    /// # Returns
    /// * `&mut [f64; N]` - A mutable reference to the coordinates of the vector.
    pub fn coords_mut(&mut self) -> &mut [f64; N] {
        &mut self.coords
    }
    /// Sets the coordinates of the vector.
    /// # Arguments
    /// * `coords` - The new coordinates of the vector.
    /// # Panics
    /// * If the coordinates cannot be converted to [f64].
    pub fn set_coords<T: ToPrimitive>(&mut self, coords: [T; N]) {
        self.coords = coords.map(|x| x.to_f64().expect("Coordinates must be convertible to f64."));
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
    ///
    /// let point1 = Point::new([1, 2, 3]);
    /// let point2 = Point::new([4, 5, 6]);
    ///
    /// let vector = Vector::from_points(point1, point2);
    /// assert_eq!(*vector.coords(), [3.0, 3.0, 3.0]);
    /// ```
    pub fn from_points(point1: Point<N>, point2: Point<N>) -> Self {
        let mut coords = [0.0; N];
        izip!(
            coords.iter_mut(),
            point1.coords().iter(),
            point2.coords().iter()
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
    /// If any of the vectors are zero.
    /// # Example
    /// ```
    /// use peuler::math::linalg::Vector;
    /// use std::f64::consts::FRAC_PI_2;
    ///
    /// let vector1 = Vector::new([1, 0]);
    /// let vector2 = Vector::new([0, 1]);
    /// assert_eq!(vector1.angle_between(&vector2), FRAC_PI_2);
    /// ```
    pub fn angle_between(&self, other: &Self) -> f64 {
        (self.dot_product(other) / (self.magnitude() * other.magnitude())).acos()
    }
    /// Calculates the cross product of two vectors.
    /// Only defined for 3-dimensional vectors.
    /// # Arguments
    /// * `other` - The other vector.
    /// # Returns
    /// * `Vector` - The cross product of the two vectors.
    /// # Panics
    /// If the vectors are not 3-dimensional.
    /// # Example
    /// ```
    /// use peuler::math::linalg::Vector;
    ///
    /// let vector1 = Vector::new([1, 0, 0]);
    /// let vector2 = Vector::new([0, 1, 0]);
    /// assert_eq!(*vector1.cross_product(&vector2).coords(), [0.0, 0.0, 1.0]);
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
    ///
    /// let vector1 = Vector::new([1, 2, 3]);
    /// let vector2 = Vector::new([4, 5, 6]);
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
    ///
    /// let vector = Vector::new([1, 2, 3]);
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
    ///
    /// let vector = Vector::new([1, 2, 3]);
    /// assert_eq!(*vector.normalize().coords(), [0.2672612419124244, 0.5345224838248488, 0.8017837257372732]);
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
impl<const N: usize, T: ToPrimitive> Mul<T> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut coords = self.coords;
        coords.iter_mut().for_each(|coord| *coord *= rhs.to_f64().expect("Scalar must be convertible to f64."));
        Self { coords }
    }
}
