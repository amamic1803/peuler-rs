//! Geometry.

use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub mod dim2;
pub mod dim3;

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

#[cfg(test)]
mod tests {
    use super::*;

    // point

    #[test]
    #[should_panic]
    fn point_new_zero_dimensions() {
        //! Test that [Point::new] panics with zero dimensional input.
        Point::<i32, 0>::new([]);
    }

    #[test]
    fn point_new() {
        //! Test [Point::new].
        let p = Point::new([1, 2]);
        assert_eq!(*p, [1, 2]);
    }

    #[test]
    fn point_primitive_types() {
        //! Test that [Point] works with primitive types.
        // unsigned types
        assert_eq!(*Point::new([1u8, 2u8]), [1u8, 2u8]);
        assert_eq!(*Point::new([1u16, 2u16]), [1u16, 2u16]);
        assert_eq!(*Point::new([1u32, 2u32]), [1u32, 2u32]);
        assert_eq!(*Point::new([1u64, 2u64]), [1u64, 2u64]);
        assert_eq!(*Point::new([1u128, 2u128]), [1u128, 2u128]);
        assert_eq!(*Point::new([1usize, 2usize]), [1usize, 2usize]);
        // signed types
        assert_eq!(*Point::new([1i8, 2i8]), [1i8, 2i8]);
        assert_eq!(*Point::new([1i16, 2i16]), [1i16, 2i16]);
        assert_eq!(*Point::new([1i32, 2i32]), [1i32, 2i32]);
        assert_eq!(*Point::new([1i64, 2i64]), [1i64, 2i64]);
        assert_eq!(*Point::new([1i128, 2i128]), [1i128, 2i128]);
        assert_eq!(*Point::new([1isize, 2isize]), [1isize, 2isize]);
        // floating point types
        assert_eq!(*Point::new([1f32, 2f32]), [1f32, 2f32]);
        assert_eq!(*Point::new([1f64, 2f64]), [1f64, 2f64]);
    }

    #[test]
    fn point_dereference() {
        //! Test that [Point] correctly implements
        //! [Deref] and [DerefMut].
        let mut p = Point::new([1, 2]);
        assert_eq!(*p, [1, 2]);
        *p =[1, 3];
        assert_eq!(*p, [1, 3]);
    }

    #[test]
    fn point_add() {
        //! Test that addition works for [Point].
        let p = Point::new([1, 2]);
        let p2 = Point::new([1, 3]);
        let mut p3 = p + p2;
        assert_eq!(*p3, [2, 5]);
        p3 += Point::new([2, 3]);
        assert_eq!(*p3, [4, 8]);
    }

    #[test]
    fn point_sub() {
        //! Test that subtraction works for [Point].
        let p = Point::new([1, 2]);
        let p2 = Point::new([1, 3]);
        let mut p3 = p - p2;
        assert_eq!(*p3, [0, -1]);
        p3 -= Point::new([2, 3]);
        assert_eq!(*p3, [-2, -4]);
    }

    #[test]
    fn point_mul() {
        //! Test that multiplication works for [Point].
        let p = Point::new([1, 2]);
        let p2 = Point::new([1, 3]);
        let mut p3 = p * p2;
        assert_eq!(*p3, [1, 6]);
        p3 *= Point::new([2, 3]);
        assert_eq!(*p3, [2, 18]);
    }

    #[test]
    fn point_div() {
        //! Test that division works for [Point].
        let p = Point::new([24, 18]);
        let p2 = Point::new([1, 2]);
        let mut p3 = p / p2;
        assert_eq!(*p3, [24, 9]);
        p3 /= Point::new([12, 3]);
        assert_eq!(*p3, [2, 3]);
    }

    #[test]
    fn point_neg() {
        //! Test that negation works for [Point].
        let p = Point::new([1, 2]);
        let p2 = -p;
        assert_eq!(*p2, [-1, -2]);
    }

    #[test]
    fn point_mul_primitives() {
        //! Test that multiplication with scalars works for [Point].
        let mut p = Point::new([1, 2]);
        assert_eq!(*(p * 2), [2, 4]);
        p *= 3;
        assert_eq!(*p, [3, 6]);
    }

    #[test]
    fn point_mul_primitives_reverse() {
        //! Test that multiplication with [Point] works for scalars.
        let p = Point::new([1, 2]);
        let p2: Point<i32, 2> = 2 * p;
        assert_eq!(*p2, [2, 4]);
    }

    #[test]
    fn point_div_primitives() {
        //! Test that division with scalars works for [Point].

        let mut p = Point::new([15, 21]);
        assert_eq!(*(p / 3), [5, 7]);
        p /= 5;
        assert_eq!(*p, [3, 4]);
    }
}