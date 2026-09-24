//! Modular arithmetic.

use crate::core::{gcd, gcd_extended};
use num_traits::{ConstOne, ConstZero, Euclid, PrimInt, Signed};
use std::borrow::Borrow;

#[cfg_attr(doc, katexit::katexit)]
/// Congruence relation.
///
/// A congruence relation is an equation that states that two integers
/// are congruent modulo a third integer.
/// Two integers $x$ and $y$ are said to be congruent modulo $n$
/// if they have the same remainder $a$ when divided by $n$.
/// This is written as:
/// $$ x \equiv a \pmod n \\\\
///    y \equiv a \pmod n
/// $$
/// # Example
/// ```
/// use pmath::numth::modular::Congruence;
///
/// let c1 = Congruence::new(3, 5); // 3 === 3 (mod 5)
/// let c2 = Congruence::new(8, 5); // 8 === 3 (mod 5)
/// assert_eq!(*c1.a(), 3);
/// assert_eq!(*c1.n(), 5);
/// assert_eq!(*c2.a(), 3);
/// assert_eq!(*c2.n(), 5);
/// assert_eq!(c1, c2);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Congruence<T> {
    a: T,
    n: T,
}
impl<T> Congruence<T>
where
    T: PartialOrd + Euclid + ConstZero + PrimInt + ConstOne,
{
    /// Create a new [Congruence].
    /// # Arguments
    /// * `a` - The remainder.
    /// * `n` - The modulus (must be positive).
    /// # Panics
    /// * If `n` is not positive.
    pub fn new(a: T, n: T) -> Self {
        if n <= T::ZERO {
            panic!("Modulus must be positive.");
        }
        Self {
            a: a.rem_euclid(&n),
            n,
        }
    }

    /// Get the remainder of the congruence relation.
    ///
    /// This might not be the same as the $a$ provided to [Congruence::new]
    /// as it is reduced modulo $n$.
    /// # Returns
    /// * The remainder.
    pub const fn a(&self) -> &T {
        &self.a
    }

    /// Get a mutable reference to the remainder of the congruence relation.
    ///
    /// This might not be the same as the $a$ provided to [Congruence::new] as it is reduced modulo $n$.
    /// # Returns
    /// * A mutable reference to the remainder.
    pub const fn a_mut(&mut self) -> &mut T {
        &mut self.a
    }

    /// Get the modulus of the congruence relation.
    /// # Returns
    /// * The modulus.
    pub const fn n(&self) -> &T {
        &self.n
    }

    /// Get a mutable reference to the modulus of the congruence relation.
    /// # Returns
    /// * A mutable reference to the modulus.
    pub const fn n_mut(&mut self) -> &mut T {
        &mut self.n
    }

    /// Compute the Chinese Remainder Theorem for this congruence and another.
    ///
    /// The explanation is given in the [crt] function.
    /// # Arguments
    /// * `other` - The other congruence.
    /// # Returns
    /// * The solution to the system of congruences and the modulus up to which the solution is unique, or [None] if no solution exists.
    pub fn crt(&self, other: &Congruence<T>) -> Option<(T, T)>
    where
        T: Signed
    {
        let a1 = self.a;
        let n1 = self.n;
        let a2 = other.a;
        let n2 = other.n;

        let d = gcd(n1, n2);

        if (a2 - a1).rem_euclid(&d) != T::ZERO {
            return None;
        }

        let n1_reduced = n1 / d;
        let n2_reduced = n2 / d;

        let (_, m1, m2) = gcd_extended(n1_reduced, n2_reduced);

        let new_n = n1_reduced * n2;
        Some(((a1 * n2_reduced * m2 + a2 * n1_reduced * m1).rem_euclid(&new_n), new_n))
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// Chinese Remainder Theorem.
///
/// The Chinese Remainder Theorem states that if one has a system of simultaneous congruences
/// with pairwise coprime moduli,
/// $$
///     x \equiv a_1 \pmod{n_1} \\\\
///     x \equiv a_2 \pmod{n_2} \\\\
///     \vdots \\\\
///     x \equiv a_k \pmod{n_k}
/// $$
/// then there exists a unique solution $x$ modulo $N = n_1 n_2 \cdots n_k$.
/// If the moduli are not coprime, the solution $x$ might still exist and be unique modulo $N = \text{lcm}(n_1, n_2, \ldots, n_k)$.
/// # Arguments
/// * `congruences` - An iterable of [Congruence]s.
/// # Returns
/// * The solution $x$ to the system of congruences and the modulus $N$ up to which the solution is unique, or [None] if no solution exists.
/// # Example
/// ```
/// use pmath::numth::modular::{crt, Congruence};
///
/// let congruences = [
///    Congruence::new(9, 10),
///    Congruence::new(5, 6),
/// ];
/// assert_eq!(crt(congruences), Some((29, 30)));
///
/// let congruences = [
///    Congruence::new(7i64, 19),
///    Congruence::new(6, 17),
///    Congruence::new(11, 13),
///    Congruence::new(2, 7),
///    Congruence::new(2, 5),
///    Congruence::new(1, 3),
///    Congruence::new(4, 11),
/// ];
/// assert_eq!(crt(congruences), Some((3_903_937, 4_849_845)));
/// ```
pub fn crt<T, U, V>(congruences: U) -> Option<(T, T)>
where
    U: IntoIterator<Item = V>,
    V: Borrow<Congruence<T>>,
    T: Signed + PartialOrd + Euclid + ConstZero + PrimInt + ConstOne,
{
    let mut congruences = congruences.into_iter();
    let mut current_congruence = *congruences.next()?.borrow();

    for congruence in congruences {
        current_congruence = match current_congruence.crt(congruence.borrow()) {
            Some((a, n)) => Congruence::new(a, n),
            None => return None,
        };
    }

    Some((*current_congruence.a(), *current_congruence.n()))
}

#[cfg_attr(doc, katexit::katexit)]
/// Multiplicative order.
///
/// Multiplicative order of an integer `a` modulo `n`, where `a` and `n` are coprime,
/// is the smallest positive integer `k` such that $ a^k \equiv 1 \pmod n $.
/// # Arguments
/// * `a` - The base.
/// * `n` - The modulus.
/// # Returns
/// * The multiplicative order.
/// # Panics
/// * If `a` or `n` is less than `2`.
/// * If `a` and `n` are not coprime.
/// # Example
/// ```
/// use pmath::numth::modular::ord;
///
/// // ord(3, 7) = 6
/// assert_eq!(ord(3, 7), 6);
/// ```
pub fn ord<T>(a: T, n: T) -> T
where
    T: PrimInt + ConstOne,
{
    let t2 = T::from(2).unwrap();
    if n < t2 || a < t2 {
        panic!("a and n must be greater than or equal to 2.");
    }
    // we want the smallest k so that a^k ≡ 1 (mod n)
    // a^k (mod n) = ((a^(k-1) (mod n)) * a) (mod n)
    // example: 8^2 mod 7 = ((8 mod 7) * 8) mod 7
    // k <= n - 1 (Fermat's little theorem)

    let mut result = T::ONE;
    let mut k = T::ONE;
    while k < n {
        result = (result * a) % n;
        if result == T::ONE {
            return k;
        }
        k = k + T::ONE;
    }

    // because of fermat's little theorem, if a and n are coprime,
    // the multiplicative order must exist because a^(n-1) ≡ 1 (mod n)
    // if we reach this point, it means we didn't find the order,
    // so a and n are not coprime
    panic!("a and n are not coprime.");
}

#[cfg(test)]
mod tests {
    use crate::numth::prime::coprime;
    use super::*;

    #[test]
    fn congruence_new() {
        //! Test that [Congruence::new] creates a congruence relation correctly.
        let c = Congruence::new(3, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);

        let c = Congruence::new(8, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);
    }

    #[test]
    fn congruence_new_nonpositive_modulus() {
        //! Test that [Congruence::new] panics when the modulus is not positive.
        let result = std::panic::catch_unwind(|| Congruence::new(3, 0));
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| Congruence::new(3, -5));
        assert!(result.is_err());
    }

    #[test]
    fn congruence_primitive_types() {
        //! Test that [Congruence] works with various primitive integer types.

        // unsigned types
        let c = Congruence::new(3u8, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);
        let c = Congruence::new(8u16, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);
        let c = Congruence::new(8u32, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);
        let c = Congruence::new(8u64, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);
        let c = Congruence::new(8u128, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);
        let c = Congruence::new(8usize, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);

        // signed types
        let c = Congruence::new(3i8, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);
        let c = Congruence::new(8i16, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);
        let c = Congruence::new(8i32, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);
        let c = Congruence::new(8i64, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);
        let c = Congruence::new(8i128, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);
        let c = Congruence::new(8isize, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);
    }

    #[test]
    fn congruence_ref() {
        //! Test that methods for getting references to the remainder and modulus work correctly.
        let mut c = Congruence::new(3, 5);
        assert_eq!(*c.a(), 3);
        assert_eq!(*c.n(), 5);

        *c.a_mut() = 4;
        *c.n_mut() = 6;
        assert_eq!(*c.a(), 4);
        assert_eq!(*c.n(), 6);
    }

    #[test]
    fn congruence_crt() {
        //! Test that [Congruence::crt] computes the Chinese Remainder Theorem correctly.
        let c1 = Congruence::new(3, 5);
        let c2 = Congruence::new(2, 7);
        assert_eq!(c1.crt(&c2), Some((23, 35)));

        let c1 = Congruence::new(1, 4);
        let c2 = Congruence::new(3, 6);
        assert_eq!(c1.crt(&c2), Some((9, 12)));

        let c1 = Congruence::new(2, 6);
        let c2 = Congruence::new(4, 8);
        assert_eq!(c1.crt(&c2), Some((20, 24)));

        let c1 = Congruence::new(1, 4);
        let c2 = Congruence::new(2, 4);
        assert_eq!(c1.crt(&c2), None);
    }

    #[test]
    fn crt_verify() {
        //! Verify that [crt] computes the Chinese Remainder Theorem correctly for multiple congruences.
        let congruences = [
            Congruence::new(3, 5),
            Congruence::new(2, 7),
            Congruence::new(1, 9),
        ];
        assert_eq!(crt(congruences), Some((163, 315)));

        let congruences = [
            Congruence::new(1, 4),
            Congruence::new(3, 6),
            Congruence::new(5, 8),
        ];
        assert_eq!(crt(congruences), Some((21, 24)));

        let congruences = [
            Congruence::new(2, 6),
            Congruence::new(4, 8),
            Congruence::new(6, 10),
        ];
        assert_eq!(crt(congruences), Some((116, 120)));

        let congruences = [
            Congruence::new(1, 4),
            Congruence::new(2, 4),
            Congruence::new(3, 4),
        ];
        assert_eq!(crt(congruences), None);
    }

    #[test]
    fn crt_iterable() {
        //! Verify that [crt] works with iterables.
        let congruences = vec![
            Congruence::new(3, 5),
            Congruence::new(2, 7),
            Congruence::new(1, 9),
        ];
        assert_eq!(crt(congruences.iter()), Some((163, 315)));
        assert_eq!(crt(&congruences), Some((163, 315)));
        assert_eq!(crt(&congruences[..]), Some((163, 315)));
        assert_eq!(crt(congruences), Some((163, 315)));
    }

    #[test]
    fn ord_primitive_types() {
        //! Verify that [ord] works with various primitive integer types.

        // unsigned types
        assert_eq!(ord(3u8, 7), 6);
        assert_eq!(ord(3u16, 7), 6);
        assert_eq!(ord(3u32, 7), 6);
        assert_eq!(ord(3u64, 7), 6);
        assert_eq!(ord(3u128, 7), 6);
        assert_eq!(ord(3usize, 7), 6);

        // signed types
        assert_eq!(ord(3i8, 7), 6);
        assert_eq!(ord(3i16, 7), 6);
        assert_eq!(ord(3i32, 7), 6);
        assert_eq!(ord(3i64, 7), 6);
        assert_eq!(ord(3i128, 7), 6);
        assert_eq!(ord(3isize, 7), 6);
    }

    #[test]
    #[should_panic]
    fn ord_non_coprime() {
        //! Verify that [ord] panics when `a` and `n` are not coprime.
        ord(4, 8);
    }

    #[test]
    fn ord_less_than_two() {
        //! Verify that [ord] panics when `a` or `n` is less than 2.
        let result = std::panic::catch_unwind(|| ord(1, 7));
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| ord(3, 1));
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| ord(1, 1));
        assert!(result.is_err());
    }

    #[test]
    fn ord_verify() {
        //! Verify that [ord] computes the multiplicative order correctly.
        assert_eq!(ord(3, 7), 6);
        assert_eq!(ord(2, 5), 4);
        assert_eq!(ord(2, 9), 6);
        assert_eq!(ord(3, 10), 4);
        assert_eq!(ord(5, 11), 5);

        for n in 2..=100 {
            for a in 2..n {
                if coprime(a, n) {
                    let order = ord(a, n);
                    assert_eq!((0..order).fold(1, |acc, _| (acc * a) % n), 1);
                }
            }
        }
    }
}
