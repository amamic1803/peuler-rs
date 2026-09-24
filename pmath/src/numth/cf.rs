//! Continued fractions module.

use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;
use std::{iter, mem};
use malachite::{Integer, Rational};
use malachite::base::num::basic::traits::{One, Zero};
use num_traits::{ConstOne, ConstZero, PrimInt};

#[cfg_attr(doc, katexit::katexit)]
/// Simple continued fraction.
///
/// A simple continued fraction is a continued fraction
/// with all numerators equal to `1`.
///
/// If it is finite, it is of the form:
/// $$
///     a\_0 + \\frac{1}{a\_1 + \\frac{1}{a\_2 + \\frac{1}{\\ddots + \\frac{1}{a\_n}}}}
/// $$
/// usually represented by coefficients:
/// $$
///    \\left[ a\_0; a\_1, a\_2, \\ldots, a\_n \\right]
/// $$
/// If it is infinite, it is of the form:
/// $$
///    a\_0 + \\frac{1}{a\_1 + \\frac{1}{a\_2 + \\frac{1}{\\ddots}}}
/// $$
/// usually represented by coefficients:
/// $$
///   \\left[ a\_0; a\_1, a\_2, \\ldots \\right]
/// $$
/// # Example
/// ```
/// use pmath::numth::cf::SimpleContinuedFraction;
///
/// // custom coefficients can also be used:
/// let cf = SimpleContinuedFraction::new(vec![1, 2, 3], Some(vec![4, 5]));
/// assert_eq!(cf.non_periodic(), vec![1, 2, 3].as_slice());
/// assert_eq!(cf.periodic(), Some(vec![4, 5].as_slice()));
/// assert!(!cf.is_finite());
///
/// // if there is no periodic part, the continued fraction is finite:
/// let cf = SimpleContinuedFraction::new(vec![1, 2, 3], None);
/// assert_eq!(cf.non_periodic(), vec![1, 2, 3].as_slice());
/// assert_eq!(cf.periodic(), None);
/// assert!(cf.is_finite());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimpleContinuedFraction<T> {
    non_periodic: Vec<T>,
    periodic: Option<Vec<T>>,
}
impl<T> SimpleContinuedFraction<T>
where
    T: PrimInt + ConstZero + ConstOne + Into<Integer>,
{
    /// Create a new [SimpleContinuedFraction].
    /// # Arguments
    /// * `non_periodic` - The non-repeating coefficients of the continued fraction $( a\_0, a\_1, a\_2, \\ldots, a\_k )$.
    /// * `periodic` - The repeating coefficients of the continued fraction $( a\_{k+1}, \\ldots, a\_{k+l} )$, if any.
    /// # Returns
    /// * A new simple continued fraction with the given coefficients.
    pub fn new<U, V>(non_periodic: U, periodic: Option<U>) -> Self
    where
        U: IntoIterator<Item = V>,
        V: Borrow<T>,
    {
        let non_periodic = non_periodic.into_iter().map(|x| *x.borrow()).collect();
        let periodic = periodic.map(|p| p.into_iter().map(|x| *x.borrow()).collect());
        Self {
            non_periodic,
            periodic,
        }
    }

    /// Create a new [SimpleContinuedFraction] representing the square root of an integer.
    /// # Arguments
    /// * `n` - The integer of which the square root is to be represented as a continued fraction.
    /// # Returns
    /// * A new simple continued fraction representing the square root of the integer.
    /// # Panics
    /// * If `n` is negative.
    /// * If `n` cannot be converted to [f64].
    /// # Example
    /// ```
    /// use malachite::base::rounding_modes::RoundingMode;
    /// use malachite::base::num::conversion::traits::RoundingFrom;
    /// use pmath::numth::cf::SimpleContinuedFraction;
    ///
    /// // continued fraction of sqrt(2) = [1; 2, 2, 2, ...]
    /// //     - coefficient 1 is not repeating
    /// //     - coefficient 2 is repeating (forms a periodic part)
    /// let cf = SimpleContinuedFraction::from_sqrt(2);
    /// assert_eq!(cf.non_periodic(), vec![1].as_slice());
    /// assert_eq!(cf.periodic(), Some(vec![2].as_slice()));
    /// assert!(!cf.is_finite());
    /// assert!(f64::rounding_from(cf.convergents().nth(25).unwrap(), RoundingMode::Nearest).0 - 2.0_f64.sqrt() < 1e-6);
    ///
    /// // continued fraction of sqrt(3) = [1; 1, 2, 1, 2, ...]
    /// //     - coefficient 1 is not repeating
    /// //     - coefficients 1 and 2 are repeating (form a periodic part)
    /// let cf = SimpleContinuedFraction::from_sqrt(3);
    /// assert_eq!(cf.non_periodic(), vec![1].as_slice());
    /// assert_eq!(cf.periodic(), Some(vec![1, 2].as_slice()));
    /// assert!(!cf.is_finite());
    /// assert!(f64::rounding_from(cf.convergents().nth(25).unwrap(), RoundingMode::Nearest).0 - 3.0_f64.sqrt() < 1e-6);
    /// ```
    pub fn from_sqrt(n: T) -> Self
    where
        T: Hash,
    {
        if n < T::ZERO {
            panic!("Cannot calculate square root of a negative integer.");
        }

        // integer square root of n
        let root = T::from(n.to_f64().expect("Cannot convert n to f64.").sqrt().floor()).unwrap();

        let non_periodic = vec![root];
        let mut periodic = None;

        // if n is not a perfect square, then find the periodic part of the continued fraction
        if root * root != n {
            // vector for storing the periodic part of the continued fraction
            periodic = Some(Vec::new());

            // set for storing the rational part of the numerator and the denominator
            // used for detecting the period
            let mut set = HashSet::new();

            // rational part of the numerator, it is a negative number such that -root < num < 0
            // here it is stored as positive because calculations take into account the negative sign
            let mut num = root;
            // denominator, starts with 1
            let mut denom = T::ONE;

            // calculate the next iteration of the continued fraction
            // until the set contains the numerator and the denominator
            // which means the period is found
            while !set.contains(&(num, denom)) {
                set.insert((num, denom));

                denom = (n - num * num) / denom;
                let expanded_val = (num + root) / denom;

                // push the expanded value to the periodic part of the continued fraction
                periodic.as_mut().unwrap().push(expanded_val);

                num = denom * expanded_val - num; // -(num - denom * expanded_val)
            }
        }

        Self {
            non_periodic,
            periodic,
        }
    }

    /// Get the reference to the non-repeating coefficients of the continued fraction.
    /// # Returns
    /// * A slice of the non-repeating coefficients.
    pub fn non_periodic(&self) -> &[T] {
        &self.non_periodic
    }

    /// Get the mutable reference to the non-repeating coefficients of the continued fraction.
    /// # Returns
    /// * A mutable slice of the non-repeating coefficients.
    pub fn non_periodic_mut(&mut self) -> &mut Vec<T> {
        &mut self.non_periodic
    }

    /// Get the reference to the repeating coefficients of the continued fraction, if any.
    /// # Returns
    /// * An [Option] containing a slice of the repeating coefficients.
    pub fn periodic(&self) -> Option<&[T]> {
        self.periodic.as_deref()
    }

    /// Get the mutable reference to the repeating coefficients of the continued fraction, if any.
    /// # Returns
    /// * An [Option] containing a mutable slice of the repeating coefficients.
    pub fn periodic_mut(&mut self) -> Option<&mut Vec<T>> {
        self.periodic.as_mut()
    }

    /// Check if the continued fraction is finite.
    /// # Returns
    /// * `true` if the continued fraction is finite, `false` otherwise.
    pub fn is_finite(&self) -> bool {
        self.periodic.is_none()
    }

    /// The convergents of the continued fraction.
    ///
    /// These are the fractions that approximate the value of the continued fraction,
    /// and are generated by taking the coefficients of the continued fraction:
    /// $$
    ///     \\begin{align*}
    ///         &a\_0 \\\\
    ///         &a\_0 + \\frac{1}{a\_1} \\\\
    ///         &a\_0 + \\frac{1}{a\_1 + \\frac{1}{a\_2}} \\\\
    ///         &a\_0 + \\frac{1}{a\_1 + \\frac{1}{a\_2 + \\frac{1}{a\_3}}} \\\\
    ///         &\\vdots
    ///     \\end{align*}
    /// $$
    /// Each subsequent convergent uses one more coefficient than the previous one,
    /// therefore better approximating the value of the continued fraction.
    /// # Returns
    /// * An iterator over the convergents of the continued fraction.
    ///   If the continued fraction is finite, the iterator ends with
    ///   the exact value of the continued fraction, and
    ///   if it is infinite, the iterator continues indefinitely,
    ///   producing fractions that better and better approximate the value of the continued fraction.
    /// # Example
    /// ```
    /// use pmath::numth::cf::SimpleContinuedFraction;
    /// use malachite::rational::Rational;
    ///
    /// let cf = SimpleContinuedFraction::new(vec![1, 2], Some(vec![3, 4]));
    /// let mut convergents = cf.convergents();
    /// assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(1, 1));
    /// assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(3, 2));
    /// assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(10, 7));
    /// assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(43, 30));
    /// assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(139, 97));
    /// assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(599, 418));
    /// // ... and so on (infinitely)
    /// ```
    pub fn convergents(&self) -> impl Iterator<Item = Rational> {
        let mut prev_num = Integer::ZERO;
        let mut prev_den = Integer::ONE;
        let mut num = Integer::ONE;
        let mut den = Integer::ZERO;
        let mut values = self
            .non_periodic
            .iter()
            .chain(self.periodic.iter().flat_map(|v| v.iter().cycle()));

        iter::from_fn(move || {
            let next_value = values.next()?;
            let next_num = (*next_value).into() * &num + &prev_num;
            let next_den = (*next_value).into() * &den + &prev_den;
            prev_num = mem::replace(&mut num, next_num);
            prev_den = mem::replace(&mut den, next_den);
            Some(Rational::from_integers_ref(&num, &den))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use malachite::base::rounding_modes::RoundingMode;
    use malachite::base::num::conversion::traits::RoundingFrom;
    use assert_float_eq::assert_float_absolute_eq;

    #[test]
    fn simple_continued_fraction_new() {
        //! Test that [SimpleContinuedFraction::new] creates a new [SimpleContinuedFraction] with the given coefficients.

        let cf = SimpleContinuedFraction::new(vec![1, 2, 3], Some(vec![4, 5]));

        assert_eq!(cf.non_periodic(), vec![1, 2, 3].as_slice());
        assert_eq!(cf.periodic(), Some(vec![4, 5].as_slice()));
        assert!(!cf.is_finite());

        let cf = SimpleContinuedFraction::new(vec![1, 2, 3], None);
        assert_eq!(cf.non_periodic(), vec![1, 2, 3].as_slice());
        assert_eq!(cf.periodic(), None);
        assert!(cf.is_finite());
    }

    #[test]
    fn simple_continued_fraction_from_sqrt_verify() {
        //! Test that [SimpleContinuedFraction::from_sqrt] creates a new [SimpleContinuedFraction] representing the square root of an integer.

        let cf = SimpleContinuedFraction::from_sqrt(2);
        assert_eq!(cf.non_periodic(), vec![1].as_slice());
        assert_eq!(cf.periodic(), Some(vec![2].as_slice()));
        assert!(!cf.is_finite());

        let cf = SimpleContinuedFraction::from_sqrt(3);
        assert_eq!(cf.non_periodic(), vec![1].as_slice());
        assert_eq!(cf.periodic(), Some(vec![1, 2].as_slice()));
        assert!(!cf.is_finite());

        let cf = SimpleContinuedFraction::from_sqrt(4);
        assert_eq!(cf.non_periodic(), vec![2].as_slice());
        assert_eq!(cf.periodic(), None);
        assert!(cf.is_finite());

        for i in 2..=100 {
            let cf = SimpleContinuedFraction::from_sqrt(i);
            let approx = f64::rounding_from(cf.convergents().take(25).last().unwrap(), RoundingMode::Nearest).0;
            let actual = (i as f64).sqrt();
            assert_float_absolute_eq!(approx, actual, 1e-6);
        }
    }

    #[test]
    #[should_panic]
    fn simple_continued_fraction_from_sqrt_negative() {
        //! Test that [SimpleContinuedFraction::from_sqrt] panics when given a negative integer.

        let _ = SimpleContinuedFraction::from_sqrt(-1);
    }

    #[test]
    fn simple_continued_fraction_references() {
        //! Test that [SimpleContinuedFraction] returns the correct references to the non-repeating and repeating coefficients.

        let mut cf = SimpleContinuedFraction::new(vec![1, 2, 3], Some(vec![4, 5]));

        assert_eq!(cf.non_periodic(), vec![1, 2, 3].as_slice());
        assert_eq!(cf.periodic(), Some(vec![4, 5].as_slice()));

        *cf.non_periodic_mut() = vec![6, 7, 8];
        *cf.periodic_mut().unwrap() = vec![9, 10];

        assert_eq!(cf.non_periodic(), vec![6, 7, 8].as_slice());
        assert_eq!(cf.periodic(), Some(vec![9, 10].as_slice()));
    }

    #[test]
    fn simple_continued_fraction_is_finite() {
        //! Test that [SimpleContinuedFraction::is_finite] returns the correct value.

        let cf = SimpleContinuedFraction::new(vec![1, 2, 3], Some(vec![4, 5]));
        assert!(!cf.is_finite());

        let cf = SimpleContinuedFraction::new(vec![1, 2, 3], None);
        assert!(cf.is_finite());

        let cf = SimpleContinuedFraction::from_sqrt(2);
        assert!(!cf.is_finite());
    }

    #[test]
    fn simple_continued_fraction_convergents() {
        //! Test that [SimpleContinuedFraction::convergents] returns the correct convergents.

        let cf = SimpleContinuedFraction::new(vec![1, 2], Some(vec![3, 4]));
        let mut convergents = cf.convergents();
        assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(1, 1));
        assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(3, 2));
        assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(10, 7));
        assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(43, 30));
        assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(139, 97));
        assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(599, 418));

        let cf = SimpleContinuedFraction::new(vec![1, 2], None);
        let mut convergents = cf.convergents();
        assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(1, 1));
        assert_eq!(convergents.next().unwrap(), Rational::const_from_unsigneds(3, 2));
        assert!(convergents.next().is_none());
    }
}
