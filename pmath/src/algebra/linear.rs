//! Linear algebra.

use crate::geometry::Point;
use matrixmultiply::dgemm;
use num_traits::ToPrimitive;
use std::mem::MaybeUninit;
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

/// A vector in an N-dimensional space.
///
/// All overloaded operators are element-wise.
/// For dot product, use the [dot_product](Vector::dot_product) method.
/// For cross product, use the [cross_product](Vector::cross_product) method (only for 3D vectors).
///
/// Coordinates can be accessed directly because [Deref] and [DerefMut] traits are implemented.
/// # Example
/// ```
/// use pmath::algebra::linear::Vector;
///
/// let vector = Vector::new([1, 2, 3]);
/// assert_eq!(*vector, [1.0, 2.0, 3.0]);
///
/// let quad_vector: Vector<3> = 2 * vector * 2;
/// assert_eq!(*quad_vector, [4.0, 8.0, 12.0]);
///
/// let vector2 = Vector::new([4, 5, 6]);
/// let add_vector = vector + vector2;
/// let sub_vector = vector - vector2;
/// assert_eq!(*add_vector, [5.0, 7.0, 9.0]);
/// assert_eq!(*sub_vector, [-3.0, -3.0, -3.0]);
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
    /// * The new vector.
    /// # Panics
    /// * If the coordinates cannot be converted to [f64].
    /// * If `N` is zero.
    pub fn new<T: ToPrimitive>(coords: [T; N]) -> Self {
        if N == 0 {
            panic!("Vector cannot have zero length.");
        }
        let coords = coords.map(|x| x.to_f64().expect("Coordinates must be convertible to f64."));
        Self { coords }
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
    /// use pmath::algebra::linear::Vector;
    /// use pmath::geometry::Point;
    ///
    /// let point1 = Point::new([1, 2, 3]);
    /// let point2 = Point::new([4, 5, 6]);
    ///
    /// let vector = Vector::from_points(point1, point2);
    /// assert_eq!(*vector, [3.0, 3.0, 3.0]);
    /// ```
    pub fn from_points<T: ToPrimitive, U: ToPrimitive>(
        point1: Point<T, N>,
        point2: Point<U, N>,
    ) -> Self {
        let mut coords = [0.0; N];
        coords
            .iter_mut()
            .zip(point1.iter())
            .zip(point2.iter())
            .for_each(|((coord, x), y)| {
                let x = x
                    .to_f64()
                    .expect("Point coordinates must be convertible to f64.");
                let y = y
                    .to_f64()
                    .expect("Point coordinates must be convertible to f64.");
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
    /// use pmath::algebra::linear::Vector;
    /// use std::f64::consts::FRAC_PI_2;
    ///
    /// let vector1 = Vector::new([1, 0]);
    /// let vector2 = Vector::new([0, 1]);
    /// assert_eq!(vector1.angle_between(&vector2), FRAC_PI_2);
    /// ```
    pub fn angle_between(&self, other: &Self) -> f64 {
        let denom = self.magnitude() * other.magnitude();
        if denom == 0.0 {
            panic!("Cannot calculate the angle between a zero vector.");
        }
        (self.dot_product(other) / denom).acos()
    }

    /// Calculate the dot product of two vectors.
    /// # Arguments
    /// * `other` - The other vector.
    /// # Returns
    /// * The dot product of the two vectors.
    /// # Example
    /// ```
    /// use pmath::algebra::linear::Vector;
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
    /// use pmath::algebra::linear::Vector;
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
    /// use pmath::algebra::linear::Vector;
    ///
    /// let vector = Vector::new([1, 2, 3]);
    /// assert_eq!(*vector.normalize(), [0.2672612419124244, 0.5345224838248488, 0.8017837257372732]);
    /// ```
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        let mut new_vector = Self {
            coords: self.coords,
        };
        if magnitude == 0.0 {
            panic!("Cannot normalize a zero vector.");
        }
        new_vector /= magnitude;
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
    /// use pmath::algebra::linear::Vector;
    ///
    /// let vector1 = Vector::new([1, 0, 0]);
    /// let vector2 = Vector::new([0, 1, 0]);
    /// assert_eq!(*vector1.cross_product(&vector2), [0.0, 0.0, 1.0]);
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
        self.coords
            .iter_mut()
            .zip(other.coords.iter())
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
        self.coords
            .iter_mut()
            .zip(other.coords.iter())
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
        self.coords
            .iter_mut()
            .zip(rhs.coords.iter())
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
        self.coords
            .iter_mut()
            .zip(rhs.coords.iter())
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
        self.coords.iter_mut().for_each(|coord| *coord *= rhs);
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
        self.coords.iter_mut().for_each(|coord| *coord /= rhs);
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
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

/// A matrix with N rows and M columns.
///
/// All overloaded operators are element-wise.
/// For matrix multiplication, use the [matmul](Matrix::matmul) method.
///
/// Data can be accessed directly because [Deref] and [DerefMut] traits are implemented.
/// # Example
/// ```
/// use pmath::algebra::linear::{Matrix, Vector};
///
/// let matrix = Matrix::new([[1, 2], [3, 4]]);
/// assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
///
/// let identity = Matrix::<2, 2>::identity();
/// assert_eq!(*identity, [[1.0, 0.0], [0.0, 1.0]]);
/// let product = matrix.matmul(&identity);
/// assert_eq!(*product, [[1.0, 2.0], [3.0, 4.0]]);
///
/// let vector = Vector::new([5, 6]);
/// let result_vector = matrix * vector;
/// assert_eq!(*result_vector, [17.0, 39.0]);
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
    /// * If `N` or `M` is zero.
    pub fn new<T: ToPrimitive>(data: [[T; M]; N]) -> Self {
        if N == 0 || M == 0 {
            panic!("Matrix cannot have zero rows or columns.");
        }
        let data = data.map(|row| {
            row.map(|x| {
                x.to_f64()
                    .expect("Matrix elements must be convertible to f64.")
            })
        });
        Self { data }
    }

    /// Perform matrix multiplication.
    /// # Arguments
    /// * `rhs` - The right-hand side matrix.
    /// # Returns
    /// * The result of the matrix multiplication.
    pub fn matmul<const P: usize>(&self, rhs: &Matrix<M, P>) -> Matrix<N, P> {
        // create uninitialized matrix for the result
        let mut result_matrix: MaybeUninit<[[f64; P]; N]> = MaybeUninit::uninit();

        // matrix multiplication
        unsafe {
            dgemm(
                N,
                M,
                P,
                1.0,
                self.data[0].as_ptr(),
                M as isize,
                1,
                rhs.data[0].as_ptr(),
                P as isize,
                1,
                0.0,
                result_matrix.as_mut_ptr() as *mut f64,
                P as isize,
                1,
            );
        }

        // convert to initialized matrix
        let result_matrix: [[f64; P]; N] = unsafe { result_matrix.assume_init() };

        // return the result
        Matrix::<N, P> {
            data: result_matrix,
        }
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
    /// * The number of row swaps performed during the elimination process.
    pub fn gaussian_elimination(&mut self) -> usize {
        let mut r = 0; // current row
        let mut c = 0; // current column
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
                if i_max != r {
                    row_swaps += 1;
                    (self.data[r], self.data[i_max]) = (self.data[i_max], self.data[r]);
                }

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
    /// # Returns
    /// * The number of row swaps performed during the Gaussian elimination step.
    pub fn gauss_jordan_elimination(&mut self) -> usize {
        let row_swaps = self.gaussian_elimination();

        for i in (0..N).rev() {
            // find pivot
            let mut pivot = usize::MAX;
            for j in 0..M {
                if self.data[i][j] != 0.0 {
                    pivot = j;
                    break;
                }
            }
            if pivot == usize::MAX {
                // no pivot in this row
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

        row_swaps
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
        for (row1, row2) in self.data.iter_mut().zip(other.data.iter()) {
            for (cell1, cell2) in row1.iter_mut().zip(row2.iter()) {
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
        for (row1, row2) in self.data.iter_mut().zip(other.data.iter()) {
            for (cell1, cell2) in row1.iter_mut().zip(row2.iter()) {
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
        for (row1, row2) in self.data.iter_mut().zip(rhs.data.iter()) {
            for (cell1, cell2) in row1.iter_mut().zip(row2.iter()) {
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
        for (row1, row2) in self.data.iter_mut().zip(rhs.data.iter()) {
            for (cell1, cell2) in row1.iter_mut().zip(row2.iter()) {
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
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
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
            coords[j] = self
                .coords
                .iter()
                .zip(rhs.data.iter().map(|row| row[j]))
                .map(|(a, b)| a * b)
                .sum();
        }
        Self::Output { coords }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::assert_float_absolute_eq;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

    // vector tests

    #[test]
    fn vector_primitive_types() {
        //! Test the [Vector] works with different primitive types.

        // unsigned types
        let vector = Vector::new([1u8, 2u8, 3u8]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);
        let vector = Vector::new([1u16, 2u16, 3u16]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);
        let vector = Vector::new([1u32, 2u32, 3u32]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);
        let vector = Vector::new([1u64, 2u64, 3u64]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);
        let vector = Vector::new([1u128, 2u128, 3u128]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);
        let vector = Vector::new([1usize, 2usize, 3usize]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);

        // signed types
        let vector = Vector::new([1i8, 2i8, 3i8]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);
        let vector = Vector::new([1i16, 2i16, 3i16]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);
        let vector = Vector::new([1i32, 2i32, 3i32]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);
        let vector = Vector::new([1i64, 2i64, 3i64]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);
        let vector = Vector::new([1i128, 2i128, 3i128]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);
        let vector = Vector::new([1isize, 2isize, 3isize]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn vector_new() {
        //! Test the [Vector::new] constructor.

        let vector = Vector::new([1, 2, 3]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);

        let vector = Vector::new([1.5, 2.5, 3.5]);
        assert_eq!(*vector, [1.5, 2.5, 3.5]);

        let vector = Vector::new([1]);
        assert_eq!(*vector, [1.0]);
    }

    #[test]
    #[should_panic]
    fn vector_new_zero_length() {
        //! Test that the [Vector::new] constructor panics when `N` is zero.

        Vector::new::<i32>([]);
    }

    #[test]
    fn vector_from_points() {
        //! Test the [Vector::from_points] constructor.

        let point1 = Point::new([1, 2, 3]);
        let point2 = Point::new([4, 5, 6]);

        let vector = Vector::from_points(point1, point2);
        assert_eq!(*vector, [3.0, 3.0, 3.0]);

        let point1 = Point::new([1.5, 2.5]);
        let point2 = Point::new([4.5, 5.5]);

        let vector = Vector::from_points(point1, point2);
        assert_eq!(*vector, [3.0, 3.0]);

        let point1 = Point::new([1]);
        let point2 = Point::new([4]);

        let vector = Vector::from_points(point1, point2);
        assert_eq!(*vector, [3.0]);

        let point1 = Point::new([1, 2, 3]);
        let point2 = Point::new([1, 0, 2]);

        let vector = Vector::from_points(point1, point2);
        assert_eq!(*vector, [0.0, -2.0, -1.0]);
    }

    #[test]
    fn vector_angle_between() {
        //! Test the [Vector::angle_between] method.

        let vector1 = Vector::new([1, 0]);
        let vector2 = Vector::new([0, 1]);
        assert_float_absolute_eq!(vector1.angle_between(&vector2), FRAC_PI_2);

        let vector1 = Vector::new([1, 0, 0]);
        let vector2 = Vector::new([0, 1, 0]);
        assert_float_absolute_eq!(vector1.angle_between(&vector2), FRAC_PI_2);

        let vector1 = Vector::new([1, 0, 0]);
        let vector2 = Vector::new([0, 0, 1]);
        assert_float_absolute_eq!(vector1.angle_between(&vector2), FRAC_PI_2);

        let vector1 = Vector::new([0, 1, 0]);
        let vector2 = Vector::new([0, 0, 1]);
        assert_float_absolute_eq!(vector1.angle_between(&vector2), FRAC_PI_2);

        let vector1 = Vector::new([1, 0]);
        let vector2 = Vector::new([1, 0]);
        assert_float_absolute_eq!(vector1.angle_between(&vector2), 0.0);

        let vector1 = Vector::new([1, 0]);
        let vector2 = Vector::new([-1, 0]);
        assert_float_absolute_eq!(vector1.angle_between(&vector2), PI);

        let vector1 = Vector::new([1, 1]);
        let vector2 = Vector::new([-1, 1]);
        assert_float_absolute_eq!(vector1.angle_between(&vector2), FRAC_PI_2);

        let vector1 = Vector::new([1, 0, 0]);
        let vector2 = Vector::new([1, 1, 0]);
        assert_float_absolute_eq!(vector1.angle_between(&vector2), FRAC_PI_4);
    }

    #[test]
    #[should_panic]
    fn vector_angle_between_zero_vector() {
        //! Test that the [Vector::angle_between] method panics when any of the vectors is zero.

        let vector1 = Vector::new([0, 0]);
        let vector2 = Vector::new([1, 0]);
        vector1.angle_between(&vector2);
    }

    #[test]
    fn vector_dot_product() {
        //! Test the [Vector::dot_product] method.

        let vector1 = Vector::new([1, 2, 3]);
        let vector2 = Vector::new([4, 5, 6]);
        assert_float_absolute_eq!(vector1.dot_product(&vector2), 32.0);

        let vector1 = Vector::new([1, 0]);
        let vector2 = Vector::new([0, 1]);
        assert_float_absolute_eq!(vector1.dot_product(&vector2), 0.0);

        let vector1 = Vector::new([1, 0]);
        let vector2 = Vector::new([1, 0]);
        assert_float_absolute_eq!(vector1.dot_product(&vector2), 1.0);

        let vector1 = Vector::new([1, 0]);
        let vector2 = Vector::new([-1, 0]);
        assert_float_absolute_eq!(vector1.dot_product(&vector2), -1.0);
    }

    #[test]
    fn vector_magnitude() {
        //! Test the [Vector::magnitude] method.

        let vector = Vector::new([1, 2, 3]);
        assert_float_absolute_eq!(vector.magnitude(), 3.7416573867739413);

        let vector = Vector::new([0, 0]);
        assert_float_absolute_eq!(vector.magnitude(), 0.0);

        let vector = Vector::new([3, 4]);
        assert_float_absolute_eq!(vector.magnitude(), 5.0);

        let vector = Vector::new([1, 1, 1]);
        assert_float_absolute_eq!(vector.magnitude(), 3.0f64.sqrt());
    }

    #[test]
    fn vector_normalize() {
        //! Test the [Vector::normalize] method.

        let vector = Vector::new([1, 2, 3]);
        let normalized = vector.normalize();
        assert_float_absolute_eq!(normalized.magnitude(), 1.0);
        assert_float_absolute_eq!(normalized.coords[0], 0.2672612419124244);
        assert_float_absolute_eq!(normalized.coords[1], 0.5345224838248488);
        assert_float_absolute_eq!(normalized.coords[2], 0.8017837257372732);

        let vector = Vector::new([3, 4]);
        let normalized = vector.normalize();
        assert_float_absolute_eq!(normalized.magnitude(), 1.0);
        assert_float_absolute_eq!(normalized.coords[0], 0.6);
        assert_float_absolute_eq!(normalized.coords[1], 0.8);
    }

    #[test]
    #[should_panic]
    fn vector_normalize_zero_vector() {
        //! Test that the [Vector::normalize] method panics when the vector is zero.

        let vector = Vector::new([0, 0]);
        vector.normalize();
    }

    #[test]
    fn vector_cross_product() {
        //! Test the [Vector::cross_product] method.

        let vector1 = Vector::new([1, 0, 0]);
        let vector2 = Vector::new([0, 1, 0]);
        let result = vector1.cross_product(&vector2);
        assert_float_absolute_eq!(result[0], 0.0);
        assert_float_absolute_eq!(result[1], 0.0);
        assert_float_absolute_eq!(result[2], 1.0);

        let vector1 = Vector::new([0, 1, 0]);
        let vector2 = Vector::new([0, 0, 1]);
        let result = vector1.cross_product(&vector2);
        assert_float_absolute_eq!(result[0], 1.0);
        assert_float_absolute_eq!(result[1], 0.0);
        assert_float_absolute_eq!(result[2], 0.0);

        let vector1 = Vector::new([0, 0, 1]);
        let vector2 = Vector::new([1, 0, 0]);
        let result = vector1.cross_product(&vector2);
        assert_float_absolute_eq!(result[0], 0.0);
        assert_float_absolute_eq!(result[1], 1.0);
        assert_float_absolute_eq!(result[2], 0.0);

        let vector1 = Vector::new([1, 0, 0]);
        let vector2 = Vector::new([0, 0, 1]);
        let result = vector1.cross_product(&vector2);
        assert_float_absolute_eq!(result[0], 0.0);
        assert_float_absolute_eq!(result[1], -1.0);
        assert_float_absolute_eq!(result[2], 0.0);

        let vector1 = Vector::new([1, 2, 3]);
        let vector2 = Vector::new([4, 5, 6]);
        let result = vector1.cross_product(&vector2);
        assert_float_absolute_eq!(result[0], -3.0);
        assert_float_absolute_eq!(result[1], 6.0);
        assert_float_absolute_eq!(result[2], -3.0);
    }

    #[test]
    fn vector_dereference() {
        //! Test that the [Deref] and [DerefMut] traits work for [Vector].

        let mut vector = Vector::new([1, 2, 3]);
        assert_eq!(*vector, [1.0, 2.0, 3.0]);
        *vector = [4.0, 5.0, 6.0];
        assert_eq!(*vector, [4.0, 5.0, 6.0]);
    }

    #[test]
    fn vector_add() {
        //! Test the [Add] and [AddAssign] traits for [Vector].

        let vector1 = Vector::new([1, 2, 3]);
        let vector2 = Vector::new([4, 5, 6]);
        let result = vector1 + vector2;
        assert_eq!(*result, [5.0, 7.0, 9.0]);
        let mut vector1 = Vector::new([1, 2, 3]);
        vector1 += Vector::new([4, 5, 6]);
        assert_eq!(*vector1, [5.0, 7.0, 9.0]);
    }

    #[test]
    fn vector_sub() {
        //! Test the [Sub] and [SubAssign] traits for [Vector].

        let vector1 = Vector::new([1, 2, 3]);
        let vector2 = Vector::new([4, 5, 6]);
        let result = vector1 - vector2;
        assert_eq!(*result, [-3.0, -3.0, -3.0]);
        let mut vector1 = Vector::new([1, 2, 3]);
        vector1 -= Vector::new([4, 5, 6]);
        assert_eq!(*vector1, [-3.0, -3.0, -3.0]);
    }

    #[test]
    fn vector_mul() {
        //! Test the [Mul] and [MulAssign] traits for [Vector].

        let vector1 = Vector::new([1, 2, 3]);
        let vector2 = Vector::new([4, 5, 6]);
        let result = vector1 * vector2;
        assert_eq!(*result, [4.0, 10.0, 18.0]);
        let mut vector1 = Vector::new([1, 2, 3]);
        vector1 *= Vector::new([4, 5, 6]);
        assert_eq!(*vector1, [4.0, 10.0, 18.0]);
    }

    #[test]
    fn vector_div() {
        //! Test the [Div] and [DivAssign] traits for [Vector].

        let vector1 = Vector::new([1, 2, 3]);
        let vector2 = Vector::new([4, 5, 6]);
        let result = vector1 / vector2;
        assert_eq!(*result, [0.25, 0.4, 0.5]);
        let mut vector1 = Vector::new([1, 2, 3]);
        vector1 /= Vector::new([4, 5, 6]);
        assert_eq!(*vector1, [0.25, 0.4, 0.5]);
    }

    #[test]
    fn vector_neg() {
        //! Test the [Neg] trait for [Vector].

        let vector = Vector::new([1, 2, 3]);
        let result = -vector;
        assert_eq!(*result, [-1.0, -2.0, -3.0]);
    }

    #[test]
    fn vector_mul_primitives() {
        //! Test the [Mul] and [MulAssign] traits for [Vector] with primitive types.

        // unsigned types

        let vector = Vector::new([1, 2, 3]);
        let result = vector * 2u8;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
        let mut vector = Vector::new([1, 2, 3]);
        vector *= 2u8;
        assert_eq!(*vector, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector * 2u16;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
        let mut vector = Vector::new([1, 2, 3]);
        vector *= 2u16;
        assert_eq!(*vector, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector * 2u32;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
        let mut vector = Vector::new([1, 2, 3]);
        vector *= 2u32;
        assert_eq!(*vector, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector * 2u64;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
        let mut vector = Vector::new([1, 2, 3]);
        vector *= 2u64;
        assert_eq!(*vector, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector * 2u128;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
        let mut vector = Vector::new([1, 2, 3]);
        vector *= 2u128;
        assert_eq!(*vector, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector * 2usize;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
        let mut vector = Vector::new([1, 2, 3]);
        vector *= 2usize;
        assert_eq!(*vector, [2.0, 4.0, 6.0]);

        // signed types

        let vector = Vector::new([1, 2, 3]);
        let result = vector * 2i8;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
        let mut vector = Vector::new([1, 2, 3]);
        vector *= 2i8;
        assert_eq!(*vector, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector * 2i16;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
        let mut vector = Vector::new([1, 2, 3]);
        vector *= 2i16;
        assert_eq!(*vector, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector * 2i32;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
        let mut vector = Vector::new([1, 2, 3]);
        vector *= 2i32;
        assert_eq!(*vector, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector * 2i64;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
        let mut vector = Vector::new([1, 2, 3]);
        vector *= 2i64;
        assert_eq!(*vector, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector * 2i128;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
        let mut vector = Vector::new([1, 2, 3]);
        vector *= 2i128;
        assert_eq!(*vector, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector * 2isize;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
        let mut vector = Vector::new([1, 2, 3]);
        vector *= 2isize;
        assert_eq!(*vector, [2.0, 4.0, 6.0]);
    }

    #[test]
    fn vector_div_primitives() {
        //! Test the [Div] and [DivAssign] traits for [Vector] with primitive types.

        // unsigned types

        let vector = Vector::new([1, 2, 3]);
        let result = vector / 2u8;
        assert_eq!(*result, [0.5, 1.0, 1.5]);
        let mut vector = Vector::new([1, 2, 3]);
        vector /= 2u8;
        assert_eq!(*vector, [0.5, 1.0, 1.5]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector / 2u16;
        assert_eq!(*result, [0.5, 1.0, 1.5]);
        let mut vector = Vector::new([1, 2, 3]);
        vector /= 2u16;
        assert_eq!(*vector, [0.5, 1.0, 1.5]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector / 2u32;
        assert_eq!(*result, [0.5, 1.0, 1.5]);
        let mut vector = Vector::new([1, 2, 3]);
        vector /= 2u32;
        assert_eq!(*vector, [0.5, 1.0, 1.5]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector / 2u64;
        assert_eq!(*result, [0.5, 1.0, 1.5]);
        let mut vector = Vector::new([1, 2, 3]);
        vector /= 2u64;
        assert_eq!(*vector, [0.5, 1.0, 1.5]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector / 2u128;
        assert_eq!(*result, [0.5, 1.0, 1.5]);
        let mut vector = Vector::new([1, 2, 3]);
        vector /= 2u128;
        assert_eq!(*vector, [0.5, 1.0, 1.5]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector / 2usize;
        assert_eq!(*result, [0.5, 1.0, 1.5]);
        let mut vector = Vector::new([1, 2, 3]);
        vector /= 2usize;
        assert_eq!(*vector, [0.5, 1.0, 1.5]);

        // signed types

        let vector = Vector::new([1, 2, 3]);
        let result = vector / 2i8;
        assert_eq!(*result, [0.5, 1.0, 1.5]);
        let mut vector = Vector::new([1, 2, 3]);
        vector /= 2i8;
        assert_eq!(*vector, [0.5, 1.0, 1.5]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector / 2i16;
        assert_eq!(*result, [0.5, 1.0, 1.5]);
        let mut vector = Vector::new([1, 2, 3]);
        vector /= 2i16;
        assert_eq!(*vector, [0.5, 1.0, 1.5]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector / 2i32;
        assert_eq!(*result, [0.5, 1.0, 1.5]);
        let mut vector = Vector::new([1, 2, 3]);
        vector /= 2i32;
        assert_eq!(*vector, [0.5, 1.0, 1.5]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector / 2i64;
        assert_eq!(*result, [0.5, 1.0, 1.5]);
        let mut vector = Vector::new([1, 2, 3]);
        vector /= 2i64;
        assert_eq!(*vector, [0.5, 1.0, 1.5]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector / 2i128;
        assert_eq!(*result, [0.5, 1.0, 1.5]);
        let mut vector = Vector::new([1, 2, 3]);
        vector /= 2i128;
        assert_eq!(*vector, [0.5, 1.0, 1.5]);

        let vector = Vector::new([1, 2, 3]);
        let result = vector / 2isize;
        assert_eq!(*result, [0.5, 1.0, 1.5]);
        let mut vector = Vector::new([1, 2, 3]);
        vector /= 2isize;
        assert_eq!(*vector, [0.5, 1.0, 1.5]);
    }

    #[test]
    fn vector_mul_primitives_reverse() {
        //! Test the [Mul] trait for [Vector] with primitive types in reverse order.

        // unsigned types

        let vector = Vector::new([1, 2, 3]);
        let result: Vector<3> = 2u8 * vector;
        assert_eq!(*result, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result: Vector<3> = 2u16 * vector;
        assert_eq!(*result, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result: Vector<3> = 2u32 * vector;
        assert_eq!(*result, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result: Vector<3> = 2u64 * vector;
        assert_eq!(*result, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result: Vector<3> = 2u128 * vector;
        assert_eq!(*result, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result: Vector<3> = 2usize * vector;
        assert_eq!(*result, [2.0, 4.0, 6.0]);

        // signed types

        let vector = Vector::new([1, 2, 3]);
        let result: Vector<3> = 2i8 * vector;
        assert_eq!(*result, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result: Vector<3> = 2i16 * vector;
        assert_eq!(*result, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result: Vector<3> = 2i32 * vector;
        assert_eq!(*result, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result: Vector<3> = 2i64 * vector;
        assert_eq!(*result, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result: Vector<3> = 2i128 * vector;
        assert_eq!(*result, [2.0, 4.0, 6.0]);

        let vector = Vector::new([1, 2, 3]);
        let result: Vector<3> = 2isize * vector;
        assert_eq!(*result, [2.0, 4.0, 6.0]);
    }

    // matrix tests

    #[test]
    fn matrix_primitive_types() {
        //! Test the [Matrix] works with different primitive types.

        // unsigned types
        let matrix = Matrix::new([[1u8, 2u8], [3u8, 4u8]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
        let matrix = Matrix::new([[1u16, 2u16], [3u16, 4u16]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
        let matrix = Matrix::new([[1u32, 2u32], [3u32, 4u32]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
        let matrix = Matrix::new([[1u64, 2u64], [3u64, 4u64]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
        let matrix = Matrix::new([[1u128, 2u128], [3u128, 4u128]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
        let matrix = Matrix::new([[1usize, 2usize], [3usize, 4usize]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);

        // signed types
        let matrix = Matrix::new([[1i8, 2i8], [3i8, 4i8]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
        let matrix = Matrix::new([[1i16, 2i16], [3i16, 4i16]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
        let matrix = Matrix::new([[1i32, 2i32], [3i32, 4i32]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
        let matrix = Matrix::new([[1i64, 2i64], [3i64, 4i64]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
        let matrix = Matrix::new([[1i128, 2i128], [3i128, 4i128]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
        let matrix = Matrix::new([[1isize, 2isize], [3isize, 4isize]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
    }

    #[test]
    fn matrix_new() {
        //! Test the [Matrix::new] constructor.

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);

        let matrix = Matrix::new([[1.5, 2.5], [3.5, 4.5]]);
        assert_eq!(*matrix, [[1.5, 2.5], [3.5, 4.5]]);

        let matrix = Matrix::new([[1]]);
        assert_eq!(*matrix, [[1.0]]);
    }

    #[test]
    #[should_panic]
    fn matrix_new_zero_rows() {
        //! Test that the [Matrix::new] constructor panics when `N` is zero.

        Matrix::<0, 10>::new::<i32>([]);
    }

    #[test]
    #[should_panic]
    fn matrix_new_zero_columns() {
        //! Test that the [Matrix::new] constructor panics when `M` is zero.

        Matrix::<10, 0>::new::<i32>([[]; 10]);
    }

    #[test]
    fn matrix_from_vector() {
        //! Test the [Matrix::from_vector] constructor.

        let vector = Vector::new([1, 2, 3]);
        let matrix = Matrix::from_vector(vector);
        assert_eq!(*matrix, [[1.0], [2.0], [3.0]]);

        let vector = Vector::new([1.5, 2.5]);
        let matrix = Matrix::from_vector(vector);
        assert_eq!(*matrix, [[1.5], [2.5]]);

        let vector = Vector::new([1]);
        let matrix = Matrix::from_vector(vector);
        assert_eq!(*matrix, [[1.0]]);
    }

    #[test]
    fn matrix_transpose() {
        //! Test the [Matrix::transpose] method.

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let transposed = matrix.transpose();
        assert_eq!(*transposed, [[1.0, 3.0], [2.0, 4.0]]);

        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let transposed = matrix.transpose();
        assert_eq!(*transposed, [[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]]);
    }

    #[test]
    fn matrix_identity() {
        //! Test the [Matrix::identity] method.

        let identity = Matrix::<2, 2>::identity();
        assert_eq!(*identity, [[1.0, 0.0], [0.0, 1.0]]);

        let identity = Matrix::<3, 3>::identity();
        assert_eq!(
            *identity,
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
        );
    }

    #[test]
    fn matrix_determinant() {
        //! Test the [Matrix::determinant] method.

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        assert_float_absolute_eq!(matrix.determinant(), -2.0);

        let matrix = Matrix::new([[1, 2, 3], [0, 1, 4], [5, 6, 0]]);
        assert_float_absolute_eq!(matrix.determinant(), 1.0);
    }

    #[test]
    fn matrix_gaussian_elimination() {
        //! Test the [Matrix::gaussian_elimination] method.

        let mut matrix = Matrix::new([[2, 1, -1, 8], [-3, -1, 2, -11], [-2, 1, 2, -3]]);
        let row_swaps = matrix.gaussian_elimination();
        assert_float_absolute_eq!(matrix[0][0], -3.0);
        assert_float_absolute_eq!(matrix[0][1], -1.0);
        assert_float_absolute_eq!(matrix[0][2], 2.0);
        assert_float_absolute_eq!(matrix[0][3], -11.0);
        assert_float_absolute_eq!(matrix[1][0], 0.0);
        assert_float_absolute_eq!(matrix[1][1], 5.0 / 3.0);
        assert_float_absolute_eq!(matrix[1][2], 2.0 / 3.0);
        assert_float_absolute_eq!(matrix[1][3], 13.0 / 3.0);
        assert_float_absolute_eq!(matrix[2][0], 0.0);
        assert_float_absolute_eq!(matrix[2][1], 0.0);
        assert_float_absolute_eq!(matrix[2][2], 0.2);
        assert_float_absolute_eq!(matrix[2][3], -0.2);
        assert_eq!(row_swaps, 2);
    }

    #[test]
    fn matrix_gauss_jordan_elimination() {
        //! Test the [Matrix::gauss_jordan_elimination] method.

        let mut matrix = Matrix::new([[2, 1, -1, 8], [-3, -1, 2, -11], [-2, 1, 2, -3]]);
        let row_swaps = matrix.gauss_jordan_elimination();
        assert_float_absolute_eq!(matrix[0][0], 1.0);
        assert_float_absolute_eq!(matrix[0][1], 0.0);
        assert_float_absolute_eq!(matrix[0][2], 0.0);
        assert_float_absolute_eq!(matrix[0][3], 2.0);
        assert_float_absolute_eq!(matrix[1][0], 0.0);
        assert_float_absolute_eq!(matrix[1][1], 1.0);
        assert_float_absolute_eq!(matrix[1][2], 0.0);
        assert_float_absolute_eq!(matrix[1][3], 3.0);
        assert_float_absolute_eq!(matrix[2][0], 0.0);
        assert_float_absolute_eq!(matrix[2][1], 0.0);
        assert_float_absolute_eq!(matrix[2][2], 1.0);
        assert_float_absolute_eq!(matrix[2][3], -1.0);
        assert_eq!(row_swaps, 2);
    }

    #[test]
    fn matrix_matmul() {
        //! Test the [Matrix::matmul] method.

        let matrix1 = Matrix::new([[1, 2], [3, 4]]);
        let matrix2 = Matrix::new([[5, 6], [7, 8]]);
        let result = matrix1.matmul(&matrix2);
        assert_eq!(*result, [[19.0, 22.0], [43.0, 50.0]]);

        let matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let matrix2 = Matrix::new([[7, 8], [9, 10], [11, 12]]);
        let result = matrix1.matmul(&matrix2);
        assert_eq!(*result, [[58.0, 64.0], [139.0, 154.0]]);

        let matrix1 = Matrix::new([[2]]);
        let matrix2 = Matrix::new([[3]]);
        let result = matrix1.matmul(&matrix2);
        assert_eq!(*result, [[6.0]]);

        let matrix1 = Matrix::new([[1, 2], [3, 4]]);
        let matrix2 = Matrix::new([[5], [6]]);
        let result = matrix1.matmul(&matrix2);
        assert_eq!(*result, [[17.0], [39.0]]);

        let matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let matrix2 = Matrix::new([[7, 8, 9], [10, 11, 12], [13, 14, 15]]);
        let result = matrix1.matmul(&matrix2);
        assert_eq!(*result, [[66.0, 72.0, 78.0], [156.0, 171.0, 186.0]]);
    }

    #[test]
    fn matrix_dereference() {
        //! Test that the [Deref] and [DerefMut] traits work for [Matrix].

        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        assert_eq!(*matrix, [[1.0, 2.0], [3.0, 4.0]]);
        *matrix = [[5.0, 6.0], [7.0, 8.0]];
        assert_eq!(*matrix, [[5.0, 6.0], [7.0, 8.0]]);
    }

    #[test]
    fn matrix_add() {
        //! Test the [Add] and [AddAssign] traits for [Matrix].

        let matrix1 = Matrix::new([[1, 2], [3, 4]]);
        let matrix2 = Matrix::new([[5, 6], [7, 8]]);
        let result = matrix1 + matrix2;
        assert_eq!(*result, [[6.0, 8.0], [10.0, 12.0]]);
        let mut matrix1 = Matrix::new([[1, 2], [3, 4]]);
        matrix1 += Matrix::new([[5, 6], [7, 8]]);
        assert_eq!(*matrix1, [[6.0, 8.0], [10.0, 12.0]]);
    }

    #[test]
    fn matrix_sub() {
        //! Test the [Sub] and [SubAssign] traits for [Matrix].

        let matrix1 = Matrix::new([[1, 2], [3, 4]]);
        let matrix2 = Matrix::new([[5, 6], [7, 8]]);
        let result = matrix1 - matrix2;
        assert_eq!(*result, [[-4.0, -4.0], [-4.0, -4.0]]);
        let mut matrix1 = Matrix::new([[1, 2], [3, 4]]);
        matrix1 -= Matrix::new([[5, 6], [7, 8]]);
        assert_eq!(*matrix1, [[-4.0, -4.0], [-4.0, -4.0]]);
    }

    #[test]
    fn matrix_mul() {
        //! Test the [Mul] and [MulAssign] traits for [Matrix].

        let matrix1 = Matrix::new([[1, 2], [3, 4]]);
        let matrix2 = Matrix::new([[5, 6], [7, 8]]);
        let result = matrix1 * matrix2;
        assert_eq!(*result, [[5.0, 12.0], [21.0, 32.0]]);
        let mut matrix1 = Matrix::new([[1, 2], [3, 4]]);
        matrix1 *= Matrix::new([[5, 6], [7, 8]]);
        assert_eq!(*matrix1, [[5.0, 12.0], [21.0, 32.0]]);
    }

    #[test]
    fn matrix_div() {
        //! Test the [Div] and [DivAssign] traits for [Matrix].

        let matrix1 = Matrix::new([[1, 2], [3, 4]]);
        let matrix2 = Matrix::new([[5, 6], [7, 8]]);
        let result = matrix1 / matrix2;
        assert_eq!(*result, [[1.0 / 5.0, 2.0 / 6.0], [3.0 / 7.0, 0.5]]);
        let mut matrix1 = Matrix::new([[1, 2], [3, 4]]);
        matrix1 /= Matrix::new([[5, 6], [7, 8]]);
        assert_eq!(*matrix1, [[1.0 / 5.0, 2.0 / 6.0], [3.0 / 7.0, 0.5]]);
    }

    #[test]
    fn matrix_neg() {
        //! Test the [Neg] trait for [Matrix].

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = -matrix;
        assert_eq!(*result, [[-1.0, -2.0], [-3.0, -4.0]]);
    }

    #[test]
    fn matrix_mul_primitives() {
        //! Test the [Mul] and [MulAssign] traits for [Matrix] with primitive types.

        // unsigned types

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix * 2u8;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix *= 2u8;
        assert_eq!(*matrix, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix * 2u16;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix *= 2u16;
        assert_eq!(*matrix, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix * 2u32;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix *= 2u32;
        assert_eq!(*matrix, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix * 2u64;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix *= 2u64;
        assert_eq!(*matrix, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix * 2u128;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix *= 2u128;
        assert_eq!(*matrix, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix * 2usize;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix *= 2usize;
        assert_eq!(*matrix, [[2.0, 4.0], [6.0, 8.0]]);

        // signed types

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix * 2i8;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix *= 2i8;
        assert_eq!(*matrix, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix * 2i16;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix *= 2i16;
        assert_eq!(*matrix, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix * 2i32;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix *= 2i32;
        assert_eq!(*matrix, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix * 2i64;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix *= 2i64;
        assert_eq!(*matrix, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix * 2i128;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix *= 2i128;
        assert_eq!(*matrix, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix * 2isize;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix *= 2isize;
        assert_eq!(*matrix, [[2.0, 4.0], [6.0, 8.0]]);
    }

    #[test]
    fn matrix_div_primitives() {
        //! Test the [Div] and [DivAssign] traits for [Matrix] with primitive types.

        // unsigned types

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix / 2u8;
        assert_eq!(*result, [[0.5, 1.0], [1.5, 2.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix /= 2u8;
        assert_eq!(*matrix, [[0.5, 1.0], [1.5, 2.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix / 2u16;
        assert_eq!(*result, [[0.5, 1.0], [1.5, 2.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix /= 2u16;
        assert_eq!(*matrix, [[0.5, 1.0], [1.5, 2.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix / 2u32;
        assert_eq!(*result, [[0.5, 1.0], [1.5, 2.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix /= 2u32;
        assert_eq!(*matrix, [[0.5, 1.0], [1.5, 2.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix / 2u64;
        assert_eq!(*result, [[0.5, 1.0], [1.5, 2.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix /= 2u64;
        assert_eq!(*matrix, [[0.5, 1.0], [1.5, 2.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix / 2u128;
        assert_eq!(*result, [[0.5, 1.0], [1.5, 2.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix /= 2u128;
        assert_eq!(*matrix, [[0.5, 1.0], [1.5, 2.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix / 2usize;
        assert_eq!(*result, [[0.5, 1.0], [1.5, 2.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix /= 2usize;
        assert_eq!(*matrix, [[0.5, 1.0], [1.5, 2.0]]);

        // signed types

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix / 2i8;
        assert_eq!(*result, [[0.5, 1.0], [1.5, 2.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix /= 2i8;
        assert_eq!(*matrix, [[0.5, 1.0], [1.5, 2.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix / 2i16;
        assert_eq!(*result, [[0.5, 1.0], [1.5, 2.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix /= 2i16;
        assert_eq!(*matrix, [[0.5, 1.0], [1.5, 2.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix / 2i32;
        assert_eq!(*result, [[0.5, 1.0], [1.5, 2.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix /= 2i32;
        assert_eq!(*matrix, [[0.5, 1.0], [1.5, 2.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix / 2i64;
        assert_eq!(*result, [[0.5, 1.0], [1.5, 2.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix /= 2i64;
        assert_eq!(*matrix, [[0.5, 1.0], [1.5, 2.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix / 2i128;
        assert_eq!(*result, [[0.5, 1.0], [1.5, 2.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix /= 2i128;
        assert_eq!(*matrix, [[0.5, 1.0], [1.5, 2.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result = matrix / 2isize;
        assert_eq!(*result, [[0.5, 1.0], [1.5, 2.0]]);
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        matrix /= 2isize;
        assert_eq!(*matrix, [[0.5, 1.0], [1.5, 2.0]]);
    }

    #[test]
    fn matrix_mul_primitives_reverse() {
        //! Test the [Mul] trait for [Matrix] with primitive types in reverse order.

        // unsigned types

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result: Matrix<2, 2> = 2u8 * matrix;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result: Matrix<2, 2> = 2u16 * matrix;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result: Matrix<2, 2> = 2u32 * matrix;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result: Matrix<2, 2> = 2u64 * matrix;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result: Matrix<2, 2> = 2u128 * matrix;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result: Matrix<2, 2> = 2usize * matrix;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);

        // signed types

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result: Matrix<2, 2> = 2i8 * matrix;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result: Matrix<2, 2> = 2i16 * matrix;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result: Matrix<2, 2> = 2i32 * matrix;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result: Matrix<2, 2> = 2i64 * matrix;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result: Matrix<2, 2> = 2i128 * matrix;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let result: Matrix<2, 2> = 2isize * matrix;
        assert_eq!(*result, [[2.0, 4.0], [6.0, 8.0]]);
    }

    // test matrix and vector multiplications

    #[test]
    fn matrix_vector_mul() {
        //! Test the multiplication of a [Matrix] by a [Vector].

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let vector = Vector::new([5, 6]);
        let result = matrix * vector;
        assert_eq!(*result, [17.0, 39.0]);

        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let vector = Vector::new([7, 8, 9]);
        let result = matrix * vector;
        assert_eq!(*result, [50.0, 122.0]);

        let matrix = Matrix::new([[1, 2], [3, 4]]);
        let vector = Vector::new([5, 6]);
        let result = vector * matrix;
        assert_eq!(*result, [23.0, 34.0]);
    }

    #[test]
    fn vector_matrix_mul() {
        //! Test the multiplication of a [Vector] by a [Matrix].

        let vector = Vector::new([1, 2]);
        let matrix = Matrix::new([[3, 4], [5, 6]]);
        let result = vector * matrix;
        assert_eq!(*result, [13.0, 16.0]);

        let vector = Vector::new([1, 2]);
        let matrix = Matrix::new([[3, 4, 5], [6, 7, 8]]);
        let result = vector * matrix;
        assert_eq!(*result, [15.0, 18.0, 21.0]);

        let vector = Vector::new([1, 2, 3]);
        let matrix = Matrix::new([[4, 5], [6, 7], [8, 9]]);
        let result = vector * matrix;
        assert_eq!(*result, [40.0, 46.0]);
    }
}
