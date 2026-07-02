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
/// use pmath::numth::modular::CongruenceRelation;
///
/// let c1 = CongruenceRelation::new(3, 5); // 3 === 3 (mod 5)
/// let c2 = CongruenceRelation::new(8, 5); // 8 === 3 (mod 5)
/// assert_eq!(*c1.a(), 3);
/// assert_eq!(*c1.n(), 5);
/// assert_eq!(*c2.a(), 3);
/// assert_eq!(*c2.n(), 5);
/// assert_eq!(c1, c2);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CongruenceRelation<T> {
    a: T,
    n: T,
}
impl<T> CongruenceRelation<T>
where
    T: PartialOrd + Euclid + ConstZero,
{
    /// Create a new [CongruenceRelation].
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
    /// This might not be the same as the $a$ provided to [CongruenceRelation::new]
    /// as it is reduced modulo $n$.
    /// # Returns
    /// * The remainder.
    pub fn a(&self) -> &T {
        &self.a
    }

    /// Get the modulus of the congruence relation.
    /// # Returns
    /// * The modulus.
    pub fn n(&self) -> &T {
        &self.n
    }
}

/// Solve a system of linear congruences.
///
/// Uses the Chinese remainder theorem.
/// If a solution exists, it is unique modulo the least common multiple of the moduli.
///
/// The moduli do not need to be coprime, but if they are not, a solution might not exist.
/// # Arguments
/// * `congruences` - An iterable of [CongruenceRelation]s representing
///   the system of linear congruences.
/// # Returns
/// * [Some] with the solution if it exists,
///   or [None] if no solution exists or no congruences were provided.
/// # Panics
/// * If any of the moduli are negative.
/// # Example
/// ```
/// use pmath::numth::modular::{system_of_linear_congruences, CongruenceRelation};
///
/// let congruences = [
///    CongruenceRelation::new(9, 10),
///    CongruenceRelation::new(5, 6),
/// ];
/// assert_eq!(system_of_linear_congruences(congruences), Some(29));
///
/// let congruences = [
///    CongruenceRelation::new(7i64, 19),
///    CongruenceRelation::new(6, 17),
///    CongruenceRelation::new(11, 13),
///    CongruenceRelation::new(2, 7),
///    CongruenceRelation::new(2, 5),
///    CongruenceRelation::new(1, 3),
///    CongruenceRelation::new(4, 11),
/// ];
/// assert_eq!(system_of_linear_congruences(congruences), Some(3_903_937));
/// ```
pub fn system_of_linear_congruences<T, U, V>(congruences: U) -> Option<T>
where
    U: IntoIterator<Item = V>,
    V: Borrow<CongruenceRelation<T>>,
    T: Copy + PrimInt + ConstOne + ConstZero + Euclid + PartialOrd + Signed,
{
    let mut congruences = congruences.into_iter().map(|val| *val.borrow());

    let mut a;
    let mut n;
    match congruences.next() {
        Some(congruence) => {
            a = *congruence.a();
            n = *congruence.n();
        }
        None => return None, // no congruences provided
    }

    for congruence in congruences {
        let mut a1 = a;
        let a2 = *congruence.a();
        let mut n1 = n;
        let n2 = *congruence.n();
        let gcd = gcd(n1, n2);

        // a1 % gcd != a2 % gcd
        // this would imply that the solution x has to have 2 different remainders
        // when divided by gcd, so no solution exists
        if (a2 - a1).rem_euclid(&gcd) != T::ZERO {
            return None;
        }

        // reduce the problem
        n1 = n1 / gcd;
        a1 = a1.rem_euclid(&n1);

        let (_, m1, m2) = gcd_extended(n1, n2);

        let new_n = n1 * n2;
        a = (a1 * n2 * m2 + a2 * n1 * m1).rem_euclid(&new_n);
        n = new_n;
    }

    Some(a)
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
    use super::*;
}
