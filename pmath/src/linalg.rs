//! Linear algebra structures and operations.

use itertools::izip;
use num_traits::ToPrimitive;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A point in an N-dimensional space.
///
/// All overloaded operators are element-wise.
///
/// Coordinates can be directly accessed because [Point] implements [Deref] and [DerefMut].
/// # Example
/// ```
/// use pmath::linalg::Point;
///
/// let mut point = Point::new([1, 2, 3]);
/// assert_eq!(*point.coords(), [1, 2, 3]);
///
/// point.set_coords([4, 5, 6]);
/// assert_eq!(*point.coords(), [4, 5, 6]);
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
    pub fn new(coords: [T; N]) -> Self {
        Self { coords }
    }

    /// Get the coordinates of the point.
    /// # Returns
    /// * The coordinates of the point.
    pub fn coords(&self) -> &[T; N] {
        &self.coords
    }

    /// Get a mutable reference to the coordinates of the point.
    /// # Returns
    /// * A mutable reference to the coordinates of the point.
    pub fn coords_mut(&mut self) -> &mut [T; N] {
        &mut self.coords
    }

    /// Set the coordinates of the point.
    /// # Arguments
    /// * `coords` - The new coordinates of the point.
    pub fn set_coords(&mut self, coords: [T; N]) {
        self.coords = coords;
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
    T: for<'a> AddAssign<&'a T>
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
    T: for<'a> AddAssign<&'a T>
{
    fn add_assign(&mut self, other: Self) {
        izip!(self.coords.iter_mut(), other.coords.iter())
            .for_each(|(coord, y)| *coord += y);
    }
}
impl<T, const N: usize> Sub<Self> for Point<T, N>
where
    T: for<'a> SubAssign<&'a T>
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
    T: for<'a> SubAssign<&'a T>
{
    fn sub_assign(&mut self, other: Self) {
        izip!(self.coords.iter_mut(), other.coords.iter())
            .for_each(|(coord, y)| *coord -= y);
    }
}
impl<T, const N: usize> Mul<Self> for Point<T, N>
where
    T: for<'a> MulAssign<&'a T>
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
    T: for<'a> MulAssign<&'a T>
{
    fn mul_assign(&mut self, rhs: Self) {
        izip!(self.coords.iter_mut(), rhs.coords.iter())
            .for_each(|(coord, y)| *coord *= y);
    }
}
impl<T, const N: usize> Div<Self> for Point<T, N>
where
    T: for<'a> DivAssign<&'a T>
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
    T: for<'a> DivAssign<&'a T>
{
    fn div_assign(&mut self, rhs: Self) {
        izip!(self.coords.iter_mut(), rhs.coords.iter())
            .for_each(|(coord, y)| *coord /= y);
    }
}
impl<T, const N: usize> Neg for Point<T, N>
where
    T: Neg<Output = T> + Copy
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
    T: for<'a> MulAssign<&'a T>
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
    T: for<'a> MulAssign<&'a T>
{
    fn mul_assign(&mut self, rhs: T) {
        self.coords
            .iter_mut()
            .for_each(|coord| *coord *= &rhs);
    }
}
impl<T, const N: usize> Div<T> for Point<T, N>
where
    T: for<'a> DivAssign<&'a T>
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
    T: for<'a> DivAssign<&'a T>
{
    fn div_assign(&mut self, rhs: T) {
        self.coords
            .iter_mut()
            .for_each(|coord| *coord /= &rhs);
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
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
);

/// A vector in an N-dimensional space.
///
/// All overloaded operators are element-wise.
/// For dot product, use the [dot_product](Vector::dot_product) method.
/// For cross product, use the [cross_product](Vector::cross_product) method (only for 3D vectors).
///
/// Coordinates can be directly accessed because [Vector] implements [Deref] and [DerefMut].
/// # Example
/// ```
/// use pmath::linalg::Vector;
///
/// let vector = Vector::new([1, 2, 3]);
/// assert_eq!(*vector.coords(), [1.0, 2.0, 3.0]);
///
/// let quad_vector: Vector<3> = 2 * vector * 2;
/// assert_eq!(*quad_vector.coords(), [4.0, 8.0, 12.0]);
///
/// let vector2 = Vector::new([4, 5, 6]);
/// let add_vector = vector + vector2;
/// let sub_vector = vector - vector2;
/// assert_eq!(*add_vector.coords(), [5.0, 7.0, 9.0]);
/// assert_eq!(*sub_vector.coords(), [-3.0, -3.0, -3.0]);
/// ```
#[derive(Copy, Clone, PartialEq)]
pub struct Vector<const N: usize> {
    coords: [f64; N],
}
impl<const N: usize> Vector<N> {
    /// Create a new [Vector].
    /// # Arguments
    /// * `coords` - The coordinates of the vector.
    /// # Returns
    /// * The vector.
    /// # Panics
    /// * If the coordinates cannot be converted to [f64].
    pub fn new<T: ToPrimitive>(coords: [T; N]) -> Self {
        let coords = coords.map(|x| x.to_f64().expect("Coordinates must be convertible to f64."));
        Self { coords }
    }

    /// Get the coordinates of the vector.
    /// # Returns
    /// * The coordinates of the vector.
    pub fn coords(&self) -> &[f64; N] {
        &self.coords
    }

    /// Get a mutable reference to the coordinates of the vector.
    /// # Returns
    /// * A mutable reference to the coordinates of the vector.
    pub fn coords_mut(&mut self) -> &mut [f64; N] {
        &mut self.coords
    }

    /// Set the coordinates of the vector.
    /// # Arguments
    /// * `coords` - The new coordinates of the vector.
    /// # Panics
    /// * If the coordinates cannot be converted to [f64].
    pub fn set_coords<T: ToPrimitive>(&mut self, coords: [T; N]) {
        self.coords = coords.map(|x| x.to_f64().expect("Coordinates must be convertible to f64."));
    }

    /// Create a new vector from 2 points.
    /// # Arguments
    /// * `point1` - The starting point.
    /// * `point2` - The ending point.
    /// # Returns
    /// * The vector from `point1` to `point2`.
    /// # Panics
    /// * If the coordinates of the points cannot be converted to [f64].
    /// # Example
    /// ```
    /// use pmath::linalg::{Point, Vector};
    ///
    /// let point1 = Point::new([1, 2, 3]);
    /// let point2 = Point::new([4, 5, 6]);
    ///
    /// let vector = Vector::from_points(point1, point2);
    /// assert_eq!(*vector.coords(), [3.0, 3.0, 3.0]);
    /// ```
    pub fn from_points<T: ToPrimitive, U: ToPrimitive>(point1: Point<T, N>, point2: Point<U, N>) -> Self {
        let mut coords = [0.0; N];
        izip!(
            coords.iter_mut(),
            point1.coords().iter(),
            point2.coords().iter()
        )
        .for_each(|(coord, x, y)| {
            let x = x.to_f64().expect("Point coordinates must be convertible to f64.");
            let y = y.to_f64().expect("Point coordinates must be convertible to f64.");
            *coord = y - x
        });
        Self { coords }
    }

    /// Calculate the angle between two vectors.
    ///
    /// The angle is in radians.
    /// # Arguments
    /// * `other` - The other vector.
    /// # Returns
    /// * The angle between the two vectors.
    /// # Panics
    /// * If any of the vectors are zero.
    /// # Example
    /// ```
    /// use pmath::linalg::Vector;
    /// use std::f64::consts::FRAC_PI_2;
    ///
    /// let vector1 = Vector::new([1, 0]);
    /// let vector2 = Vector::new([0, 1]);
    /// assert_eq!(vector1.angle_between(&vector2), FRAC_PI_2);
    /// ```
    pub fn angle_between(&self, other: &Self) -> f64 {
        (self.dot_product(other) / (self.magnitude() * other.magnitude())).acos()
    }

    /// Calculate the dot product of two vectors.
    /// # Arguments
    /// * `other` - The other vector.
    /// # Returns
    /// * The dot product of the two vectors.
    /// # Example
    /// ```
    /// use pmath::linalg::Vector;
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

    /// Calculate the magnitude of the vector.
    /// # Returns
    /// * The magnitude of the vector.
    /// # Example
    /// ```
    /// use pmath::linalg::Vector;
    ///
    /// let vector = Vector::new([1, 2, 3]);
    /// assert_eq!(vector.magnitude(), 3.7416573867739413);
    /// ```
    pub fn magnitude(&self) -> f64 {
        self.coords.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }

    /// Calculate the normalized vector.
    /// # Returns
    /// * The normalized vector.
    /// # Panics
    /// * If the vector is zero.
    /// # Example
    /// ```
    /// use pmath::linalg::Vector;
    ///
    /// let vector = Vector::new([1, 2, 3]);
    /// assert_eq!(*vector.normalize().coords(), [0.2672612419124244, 0.5345224838248488, 0.8017837257372732]);
    /// ```
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        let mut new_vector = Self { coords: self.coords };
        new_vector = new_vector / magnitude;
        new_vector
    }
}
impl Vector<3> {
    /// Calculate the cross product of two vectors.
    /// # Arguments
    /// * `other` - The other vector.
    /// # Returns
    /// * The cross product of two vectors.
    /// # Example
    /// ```
    /// use pmath::linalg::Vector;
    ///
    /// let vector1 = Vector::new([1, 0, 0]);
    /// let vector2 = Vector::new([0, 1, 0]);
    /// assert_eq!(*vector1.cross_product(&vector2).coords(), [0.0, 0.0, 1.0]);
    /// ```
    pub fn cross_product(&self, other: &Self) -> Self {
        let coords = [
            self.coords[1] * other.coords[2] - self.coords[2] * other.coords[1],
            self.coords[2] * other.coords[0] - self.coords[0] * other.coords[2],
            self.coords[0] * other.coords[1] - self.coords[1] * other.coords[0],
        ];
        Self { coords }
    }
}
impl<const N: usize> Deref for Vector<N> {
    type Target = [f64; N];

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}
impl<const N: usize> DerefMut for Vector<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.coords
    }
}
impl<const N: usize> Add<Self> for Vector<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut new_vector = self;
        new_vector += other;
        new_vector
    }
}
impl<const N: usize> AddAssign<Self> for Vector<N> {
    fn add_assign(&mut self, other: Self) {
        izip!(self.coords.iter_mut(), other.coords.iter())
            .for_each(|(coord, y)| *coord += y);
    }
}
impl<const N: usize> Sub<Self> for Vector<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut new_vector = self;
        new_vector -= other;
        new_vector
    }
}
impl<const N: usize> SubAssign<Self> for Vector<N> {
    fn sub_assign(&mut self, other: Self) {
        izip!(self.coords.iter_mut(), other.coords.iter())
            .for_each(|(coord, y)| *coord -= y);
    }
}
impl<const N: usize> Mul<Self> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_vector = self;
        new_vector *= rhs;
        new_vector
    }
}
impl<const N: usize> MulAssign<Self> for Vector<N> {
    fn mul_assign(&mut self, rhs: Self) {
        izip!(self.coords.iter_mut(), rhs.coords.iter())
            .for_each(|(coord, y)| *coord *= y);
    }
}
impl<const N: usize> Div<Self> for Vector<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut new_vector = self;
        new_vector /= rhs;
        new_vector
    }
}
impl<const N: usize> DivAssign<Self> for Vector<N> {
    fn div_assign(&mut self, rhs: Self) {
        izip!(self.coords.iter_mut(), rhs.coords.iter())
            .for_each(|(coord, y)| *coord /= y);
    }
}
impl<const N: usize> Neg for Vector<N> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self *= -1.0;
        self
    }
}
impl<const N: usize, T: ToPrimitive> Mul<T> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut new_vector = self;
        new_vector *= rhs;
        new_vector
    }
}
impl<const N: usize, T: ToPrimitive> MulAssign<T> for Vector<N> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.to_f64().expect("Scalar must be convertible to f64.");
        self.coords
            .iter_mut()
            .for_each(|coord| *coord *= rhs);
    }
}
impl<const N: usize, T: ToPrimitive> Div<T> for Vector<N> {
    type Output = Vector<N>;

    fn div(self, rhs: T) -> Self::Output {
        let mut new_vector = self;
        new_vector /= rhs;
        new_vector
    }
}
impl<const N: usize, T: ToPrimitive> DivAssign<T> for Vector<N> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.to_f64().expect("Scalar must be convertible to f64.");
        self.coords
            .iter_mut()
            .for_each(|coord| *coord /= rhs);
    }
}
macro_rules! impl_mul_scalar_by_vector {
    ($($scalar:ty),*) => {
        $(
            impl<const N: usize> Mul<Vector<N>> for $scalar {
                type Output = Vector<N>;
                fn mul(self, rhs: Vector<N>) -> Self::Output {
                    rhs * self
                }
            }
        )*
    };
}
impl_mul_scalar_by_vector!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
);

/// A matrix with N rows and M columns.
///
/// All overloaded operators are element-wise.
/// For matrix multiplication, use the [mathmul](Matrix::mathmul) method.
///
/// Data can be directly accessed because [Matrix] implements [Deref] and [DerefMut].
/// # Example
/// ```
/// use pmath::linalg::{Matrix, Vector};
///
/// let matrix = Matrix::new([[1, 2], [3, 4]]);
/// assert_eq!(matrix.data(), &[[1.0, 2.0], [3.0, 4.0]]);
///
/// let identity = Matrix::<2, 2>::identity();
/// assert_eq!(identity.data(), &[[1.0, 0.0], [0.0, 1.0]]);
/// let product = matrix.mathmul(identity);
/// assert_eq!(product.data(), &[[1.0, 2.0], [3.0, 4.0]]);
///
/// let vector = Vector::new([5, 6]);
/// let result_vector = matrix * vector;
/// assert_eq!(*result_vector.coords(), [17.0, 39.0]);
/// ```
#[derive(Copy, Clone, PartialEq)]
pub struct Matrix<const N: usize, const M: usize> {
    data: [[f64; M]; N],
}
impl<const N: usize, const M: usize> Matrix<N, M> {
    /// Create a new [Matrix].
    /// # Arguments
    /// * `data` - The data of the matrix.
    /// # Returns
    /// * The new matrix.
    /// # Panics
    /// * If the data cannot be converted to [f64].
    pub fn new<T: ToPrimitive>(data: [[T; M]; N]) -> Self {
        let data = data.map(|row| row.map(|x| x.to_f64().expect("Matrix elements must be convertible to f64.")));
        Self { data }
    }

    /// Get the data of the matrix.
    /// # Returns
    /// * The data of the matrix.
    pub fn data(&self) -> &[[f64; M]; N] {
        &self.data
    }

    /// Get a mutable reference to the data of the matrix.
    /// # Returns
    /// * A mutable reference to the data of the matrix.
    pub fn data_mut(&mut self) -> &mut [[f64; M]; N] {
        &mut self.data
    }

    /// Set the data of the matrix.
    /// # Arguments
    /// * `data` - The new data of the matrix.
    /// # Panics
    /// * If the data cannot be converted to [f64].
    pub fn set_data<T: ToPrimitive>(&mut self, data: [[T; M]; N]) {
        self.data = data.map(|row| row.map(|x| x.to_f64().expect("Matrix elements must be convertible to f64.")));
    }

    /// Perform matrix multiplication.
    /// # Arguments
    /// * `rhs` - The right-hand side matrix.
    /// # Returns
    /// * The result of the matrix multiplication.
    pub fn mathmul<const P: usize>(self, rhs: Matrix<M, P>) -> Matrix<N, P> {
        let mut data = [[0.0; P]; N];

        for (i, row) in data.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                let mut sum = 0.0;
                for k in 0..M {
                    sum += self.data[i][k] * rhs.data[k][j];
                }
                *cell = sum;
            }
        }

        Matrix::<N, P> { data }
    }

    /// Transpose the matrix.
    /// # Returns
    /// * The transposed matrix.
    pub fn transpose(&self) -> Matrix<M, N> {
        let mut data = [[0.0; N]; M];
        for i in 0..N {
            for (j, row) in data.iter_mut().enumerate() {
                row[i] = self.data[i][j];
            }
        }
        Matrix::<M, N> { data }
    }

    /// Reduce the matrix to the row echelon form using Gaussian elimination.
    /// # Returns
    /// * The number of row swaps performed while performing the elimination.
    pub fn gaussian_elimination(&mut self) -> usize {
        let mut r = 0;  // current row
        let mut c = 0;  // current column
        let mut row_swaps = 0;

        while r < N && c < M {
            // find the pivot (maximum absolute value in the current column)
            let mut i_max = r;
            let mut val_max = self.data[r][c].abs();
            for i in (r + 1)..N {
                let val = self.data[i][c].abs();
                if val > val_max {
                    i_max = i;
                    val_max = val;
                }
            }

            // if the pivot is non-zero, swap the current row with the pivot row
            // and eliminate the values below the pivot in the current column
            // by adding this row multiplied by factor f to the rows below
            if val_max != 0.0 {
                row_swaps += 1;
                (self.data[r], self.data[i_max]) = (self.data[i_max], self.data[r]);

                for i in (r + 1)..N {
                    let f = self.data[i][c] / self.data[r][c];
                    self.data[i][c] = 0.0;
                    for j in (c + 1)..M {
                        self.data[i][j] -= self.data[r][j] * f;
                    }
                }

                // move to the next row
                r += 1;
            }

            // move to the next column
            c += 1;
        }

        row_swaps
    }

    /// Reduce the matrix to the reduced row echelon form using Gauss-Jordan elimination.
    pub fn gauss_jordan_elimination(&mut self) {
        self.gaussian_elimination();

        for i in (0..N).rev() {
            // find pivot
            let mut pivot = usize::MAX;
            for j in 0..M {
                if self.data[i][j] != 0.0 {
                    pivot = j;
                    break;
                }
            }
            if pivot == usize::MAX {  // no pivot in this row
                continue;
            }
            if self.data[i][pivot] != 1.0 {
                let f = 1.0 / self.data[i][pivot];
                self.data[i][pivot] = 1.0;
                for j in (pivot + 1)..M {
                    self.data[i][j] *= f;
                }
            }

            // eliminate the pivot column
            for k in (0..i).rev() {
                let f = self.data[k][pivot] / self.data[i][pivot];
                self.data[k][pivot] = 0.0;
                for j in (pivot + 1)..M {
                    self.data[k][j] -= self.data[i][j] * f;
                }
            }
        }
    }
}
impl<const N: usize> Matrix<N, 1> {
    /// Create a new column [Matrix] from a [Vector].
    /// # Arguments
    /// * `vector` - The vector to create the matrix from.
    /// # Returns
    /// * The new matrix.
    pub fn from_vector(vector: Vector<N>) -> Self {
        let mut data = [[0.0; 1]; N];
        for (i, row) in data.iter_mut().enumerate() {
            row[0] = vector.coords[i];
        }
        Self { data }
    }
}
impl<const N: usize> Matrix<N, N> {
    /// Create an identity [Matrix].
    /// # Returns
    /// * The identity matrix.
    pub fn identity() -> Self {
        let mut data = [[0.0; N]; N];
        for (i, row) in data.iter_mut().enumerate() {
            row[i] = 1.0;
        }
        Self { data }
    }

    /// Calculate the determinant of the matrix.
    /// # Returns
    /// * The determinant of the matrix.
    pub fn determinant(&self) -> f64 {
        let mut mat = *self;
        let mut det = 1.0;
        let row_swaps = mat.gaussian_elimination();
        if !row_swaps.is_multiple_of(2) {
            det = -det;
        }
        for i in 0..N {
            det *= mat.data[i][i];
        }
        det
    }
}
impl<const N: usize, const M: usize> Deref for Matrix<N, M> {
    type Target = [[f64; M]; N];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<const N: usize, const M: usize> DerefMut for Matrix<N, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl<const N: usize, const M: usize> Add<Self> for Matrix<N, M> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut new_matrix = self;
        new_matrix += other;
        new_matrix
    }
}
impl<const N: usize, const M: usize> AddAssign<Self> for Matrix<N, M> {
    fn add_assign(&mut self, other: Self) {
        for (row1, row2) in izip!(self.data.iter_mut(), other.data.iter()) {
            for (cell1, cell2) in izip!(row1.iter_mut(), row2.iter()) {
                *cell1 += cell2;
            }
        }
    }
}
impl<const N: usize, const M: usize> Sub<Self> for Matrix<N, M> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut new_matrix = self;
        new_matrix -= other;
        new_matrix
    }
}
impl<const N: usize, const M: usize> SubAssign<Self> for Matrix<N, M> {
    fn sub_assign(&mut self, other: Self) {
        for (row1, row2) in izip!(self.data.iter_mut(), other.data.iter()) {
            for (cell1, cell2) in izip!(row1.iter_mut(), row2.iter()) {
                *cell1 -= cell2;
            }
        }
    }
}
impl<const N: usize, const M: usize> Mul<Self> for Matrix<N, M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_matrix = self;
        new_matrix *= rhs;
        new_matrix
    }
}
impl<const N: usize, const M: usize> MulAssign<Self> for Matrix<N, M> {
    fn mul_assign(&mut self, rhs: Self) {
        for (row1, row2) in izip!(self.data.iter_mut(), rhs.data.iter()) {
            for (cell1, cell2) in izip!(row1.iter_mut(), row2.iter()) {
                *cell1 *= cell2;
            }
        }
    }
}
impl<const N: usize, const M: usize> Div<Self> for Matrix<N, M> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut new_matrix = self;
        new_matrix /= rhs;
        new_matrix
    }
}
impl<const N: usize, const M: usize> DivAssign<Self> for Matrix<N, M> {
    fn div_assign(&mut self, rhs: Self) {
        for (row1, row2) in izip!(self.data.iter_mut(), rhs.data.iter()) {
            for (cell1, cell2) in izip!(row1.iter_mut(), row2.iter()) {
                *cell1 /= cell2;
            }
        }
    }
}
impl<const N: usize, const M: usize> Neg for Matrix<N, M> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self *= -1.0;
        self
    }
}
impl<const N: usize, const M: usize, T: ToPrimitive> Mul<T> for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut new_matrix = self;
        new_matrix *= rhs;
        new_matrix
    }
}
impl<const N: usize, const M: usize, T: ToPrimitive> MulAssign<T> for Matrix<N, M> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.to_f64().expect("Scalar must be convertible to f64.");
        for row in self.data.iter_mut() {
            for cell in row.iter_mut() {
                *cell *= rhs;
            }
        }
    }
}
impl<const N: usize, const M: usize, T: ToPrimitive> Div<T> for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn div(self, rhs: T) -> Self::Output {
        let mut new_matrix = self;
        new_matrix /= rhs;
        new_matrix
    }
}
impl<const N: usize, const M: usize, T: ToPrimitive> DivAssign<T> for Matrix<N, M> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.to_f64().expect("Scalar must be convertible to f64.");
        for row in self.data.iter_mut() {
            for cell in row.iter_mut() {
                *cell /= rhs;
            }
        }
    }
}
macro_rules! impl_mul_scalar_by_matrix {
    ($($scalar:ty),*) => {
        $(
            impl<const N: usize, const M: usize> Mul<Matrix<N, M>> for $scalar  {
                type Output = Matrix<N, M>;
                fn mul(self, rhs: Matrix<N, M>) -> Self::Output {
                    rhs * self
                }
            }
        )*
    }
}
impl_mul_scalar_by_matrix!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
);

impl<const N: usize, const M: usize> Mul<Vector<M>> for Matrix<N, M> {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<M>) -> Self::Output {
        let mut coords = [0.0; N];
        for (i, row) in self.data.iter().enumerate() {
            coords[i] = row.iter().zip(rhs.coords.iter()).map(|(a, b)| a * b).sum();
        }
        Self::Output { coords }
    }
}
impl<const N: usize, const M: usize> Mul<Matrix<N, M>> for Vector<N> {
    type Output = Vector<M>;

    fn mul(self, rhs: Matrix<N, M>) -> Self::Output {
        let mut coords = [0.0; M];
        for j in 0..M {
            coords[j] = self.coords.iter().zip(rhs.data.iter().map(|row| row[j])).map(|(a, b)| a * b).sum();
        }
        Self::Output { coords }
    }
}
