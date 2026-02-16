//! Probability distributions.

use num_traits::{ConstOne, PrimInt, ToPrimitive};
use rand::Rng;
use rand::RngExt;
use rand::distr::Distribution as RandDistribution;
use rand::distr::uniform::SampleUniform;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

#[cfg_attr(doc, katexit::katexit)]
/// A trait representing a generic probability distribution.
///
/// A random variable $X$ is said to follow a distribution $\\mathcal{D}$
/// (denoted as $X \\sim \\mathcal{D}$) if the probability of $X$
/// taking a value $x$ is given by the distribution functions.
pub trait Distribution<T>: RandDistribution<T> {
    /// Cumulative distribution function (CDF).
    ///
    /// The CDF of a random variable $X$ is defined as:
    /// $$
    ///     F(x) = P(X \\leq x)
    /// $$
    /// # Arguments
    /// * `x` - The value at which to evaluate the CDF.
    /// # Returns
    /// * The value of the CDF at `x`.
    /// # Panics
    /// * If `x` cannot be converted to [f64].
    fn cdf<U: ToPrimitive>(&self, x: U) -> f64;

    /// Mean of the distribution.
    ///
    /// Also known as the expected value or expectation,
    /// the mean is a measure of the central tendency of the distribution.
    ///
    /// The mean of a discrete random variable $X$ is defined as:
    /// $$
    ///     \mu = E\[X\] = \\sum_{i} x_i \\cdot P(X = x_i)
    /// $$
    /// The mean of a continuous random variable $X$ is defined as:
    /// $$
    ///    \mu = E\[X\] = \\int_{-\\infty}^{\\infty} x \\cdot f(x) dx
    /// $$
    /// # Returns
    /// * An [Option] with the mean of the distribution if it is defined.
    fn mean(&self) -> Option<f64>;

    /// Variance of the distribution.
    ///
    /// The variance is a measure of the spread of the distribution,
    /// which quantifies how much the values of the random variable differ from the mean.
    /// The variance of a random variable $X$ is defined as:
    /// $$
    ///     \\operatorname{Var}\[X\] = \\sigma^2 = E\[(X - \\mu)^2\] = E\[X^2\] - \\mu^2 = E\[X^2\] - (E\[X\])^2
    /// $$
    /// # Returns
    /// * An [Option] with the variance of the distribution if it is defined.
    fn variance(&self) -> Option<f64>;

    /// Standard deviation of the distribution.
    ///
    /// The standard deviation is the square root of the variance,
    /// which provides a measure of the spread of the distribution in the same units as the random variable.
    /// The standard deviation of a random variable $X$ is defined as:
    /// $$
    ///     \\sigma = \\sqrt{\\operatorname{Var}\[X\]} = \\sqrt{E\[(X - \\mu)^2\]} = \\sqrt{E\[X^2\] - (E\[X\])^2}
    /// $$
    /// # Returns
    /// * An [Option] with the standard deviation of the distribution if it is defined.
    fn stddev(&self) -> Option<f64> {
        self.variance().map(|v| v.sqrt())
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// A trait representing a discrete probability distribution.
pub trait DiscreteDistribution<T>: Distribution<T> {
    /// Probability mass function (PMF).
    ///
    /// The PMF of a discrete random variable $X$ is defined as:
    /// $$
    ///     f(x) = P(X = x)
    /// $$
    /// The important property of a PMF is:
    /// $$
    ///     \\sum_{x} f(x) = 1
    /// $$
    /// # Arguments
    /// * `x` - The value at which to evaluate the PMF.
    /// # Returns
    /// * The value of the PMF at `x`.
    fn pmf(&self, x: T) -> f64;
}

#[cfg_attr(doc, katexit::katexit)]
/// A trait representing a continuous probability distribution.
pub trait ContinuousDistribution<T>: Distribution<T> {
    /// Probability density function (PDF).
    ///
    /// The PDF of a continuous random variable $X$ tells the probability
    /// of $X$ taking a value in an infinitesimally small interval around $x$.
    /// The important property of a PDF is:
    /// $$
    ///     \\int_{-\\infty}^{\\infty} f(x) dx = 1
    /// $$
    /// # Arguments
    /// * `x` - The value at which to evaluate the PDF.
    /// # Returns
    /// * The value of the PDF at `x`.
    /// # Panics
    /// * If `x` cannot be converted to [f64].
    fn pdf<U: ToPrimitive>(&self, x: U) -> f64;
}

#[cfg_attr(doc, katexit::katexit)]
/// A continuous uniform distribution.
///
/// The continuous uniform distribution $\\mathcal{U}(a, b)$
/// is a distribution where all values in the interval $\[a, b\]$
/// are equally likely.
/// # Example
/// ```
/// use pmath::probability::distributions::{ContinuousUniform, Distribution, ContinuousDistribution};
///
/// let dist = ContinuousUniform::new(0.0, 1.0);
/// assert!((dist.cdf(0.5) - 0.5).abs() < 1e-10);
/// assert!((dist.pdf(0.5) - 1.0).abs() < 1e-10);
/// assert!((dist.mean().unwrap() - 0.5).abs() < 1e-10);
/// assert!((dist.variance().unwrap() - 1.0 / 12.0).abs() < 1e-10);
/// assert!((dist.stddev().unwrap() - (1.0 / 12.0f64).sqrt()).abs() < 1e-10);
/// ```
pub struct ContinuousUniform {
    a: f64,
    b: f64,
}
impl ContinuousUniform {
    /// Create a new continuous uniform distribution.
    /// # Arguments
    /// * `a` - The minimum value of the distribution.
    /// * `b` - The maximum value of the distribution.
    /// # Returns
    /// * A new [ContinuousUniform] distribution.
    /// # Panics
    /// * If `a` cannot be converted to [f64].
    /// * If `b` cannot be converted to [f64].
    /// * If `a` is greater than `b`.
    pub fn new<T, U>(a: T, b: U) -> Self
    where
        T: ToPrimitive,
        U: ToPrimitive,
    {
        let a = a.to_f64().expect("a cannot be converted to f64");
        let b = b.to_f64().expect("b cannot be converted to f64");
        if a > b {
            panic!("a must be less than or equal to b.");
        }
        Self { a, b }
    }

    /// Get the minimum value of the distribution.
    /// # Returns
    /// * The minimum value of the distribution.
    pub fn a(&self) -> f64 {
        self.a
    }

    /// Get the maximum value of the distribution.
    /// # Returns
    /// * The maximum value of the distribution.
    pub fn b(&self) -> f64 {
        self.b
    }
}
impl Distribution<f64> for ContinuousUniform {
    fn cdf<U: ToPrimitive>(&self, x: U) -> f64 {
        let x_f64 = x.to_f64().expect("x cannot be converted to f64");
        if x_f64 < self.a {
            0.0
        } else if x_f64 > self.b {
            1.0
        } else {
            (x_f64 - self.a) / (self.b - self.a)
        }
    }

    fn mean(&self) -> Option<f64> {
        Some((self.a + self.b) / 2.0)
    }

    fn variance(&self) -> Option<f64> {
        Some((self.b - self.a).powi(2) / 12.0)
    }
}
impl RandDistribution<f64> for ContinuousUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        rng.random_range(self.a..=self.b)
    }
}
impl ContinuousDistribution<f64> for ContinuousUniform {
    fn pdf<U: ToPrimitive>(&self, x: U) -> f64 {
        let x_f64 = x.to_f64().expect("x cannot be converted to f64");
        if x_f64 < self.a || x_f64 > self.b {
            0.0
        } else {
            1.0 / (self.b - self.a)
        }
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// A discrete uniform distribution.
///
/// A discrete uniform distribution with parameters $a$ and $b$ is a distribution
/// $$
///     \\begin{pmatrix}
///         a & a + 1 & a + 2 & \\ldots & b \\\\
///         \\frac{1}{b - a + 1} & \\frac{1}{b - a + 1} & \\frac{1}{b - a + 1} & \\ldots & \\frac{1}{b - a + 1}
///     \\end{pmatrix}
/// $$
/// # Example
/// ```
/// use pmath::probability::distributions::{DiscreteUniform, Distribution, DiscreteDistribution};
///
/// let dist = DiscreteUniform::new(1, 3);
/// assert!((dist.cdf(2) - 2.0 / 3.0).abs() < 1e-10);
/// assert!((dist.pmf(2) - 1.0 / 3.0).abs() < 1e-10);
/// assert!((dist.mean().unwrap() - 2.0).abs() < 1e-10);
/// assert!((dist.variance().unwrap() - 2.0 / 3.0).abs() < 1e-10);
/// assert!((dist.stddev().unwrap() - (2.0 / 3.0f64).sqrt()).abs() < 1e-10);
/// ```
pub struct DiscreteUniform<T> {
    a: T,
    b: T,
}
impl<T> DiscreteUniform<T>
where
    T: PrimInt,
{
    /// Create a new discrete uniform distribution.
    /// # Arguments
    /// * `a` - The minimum value of the distribution.
    /// * `b` - The maximum value of the distribution.
    /// # Returns
    /// * A new [DiscreteUniform] distribution.
    /// # Panics
    /// * If `a` is greater than `b`.
    pub fn new(a: T, b: T) -> Self {
        if a > b {
            panic!("a must be less than or equal to b.");
        }
        Self { a, b }
    }

    /// Get the minimum value of the distribution.
    /// # Returns
    /// * The minimum value of the distribution.
    pub fn a(&self) -> T {
        self.a
    }

    /// Get the maximum value of the distribution.
    /// # Returns
    /// * The maximum value of the distribution.
    pub fn b(&self) -> T {
        self.b
    }
}
impl<T> Distribution<T> for DiscreteUniform<T>
where
    T: SampleUniform + ConstOne + PrimInt,
{
    fn cdf<U: ToPrimitive>(&self, x: U) -> f64 {
        let x = T::from(x.to_f64().expect("x cannot be converted to f64").floor())
            .expect("x cannot be converted to T");
        if x < self.a {
            0.0
        } else if x >= self.b {
            1.0
        } else {
            (x - self.a + T::ONE).to_f64().unwrap() * self.pmf(self.a)
        }
    }

    fn mean(&self) -> Option<f64> {
        Some((self.a.to_f64().unwrap() + self.b.to_f64().unwrap()) / 2.0)
    }

    fn variance(&self) -> Option<f64> {
        let prob = self.pmf(self.a);
        let mean = self.mean().unwrap();
        let mut result = 0.0;
        let mut v = self.a;
        loop {
            result += v.to_f64().unwrap().powi(2) * prob;
            v = v + T::ONE;
            if v > self.b {
                break;
            }
        }
        Some(result - mean.powi(2))
    }
}
impl<T> RandDistribution<T> for DiscreteUniform<T>
where
    T: SampleUniform + PrimInt,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        rng.random_range(self.a..=self.b)
    }
}
impl<T> DiscreteDistribution<T> for DiscreteUniform<T>
where
    T: SampleUniform + PrimInt + ConstOne,
{
    fn pmf(&self, x: T) -> f64 {
        if x < self.a || x > self.b {
            0.0
        } else {
            1.0 / (self.b.to_f64().unwrap() - self.a.to_f64().unwrap() + 1.0)
        }
    }
}

#[cfg_attr(doc, katexit::katexit)]
/// A discrete finite distribution with custom items and probabilities.
///
/// Given a set of items $a_1, a_2, \\ldots, a_n$ with corresponding
/// probabilities $p(a_1), p(a_2), \\ldots, p(a_n)$,
/// the distribution is represented as:
/// $$
///     \\begin{pmatrix}
///         a\_1 & a\_2 & a\_3 & \\ldots & a\_n \\\\
///         p(a\_1) & p(a\_2) & p(a\_3) & \\ldots & p(a\_n)
///     \\end{pmatrix}
/// $$
/// # Example
/// ```
/// use pmath::probability::distributions::{CustomDiscreteFinite, Distribution, DiscreteDistribution};
///
/// let dist = CustomDiscreteFinite::new([(1, 0.25), (2, 0.5), (3, 0.25)]);
/// assert!((dist.cdf(1) - 0.25).abs() < 1e-10);
/// assert!((dist.cdf(2) - 0.75).abs() < 1e-10);
/// assert!((dist.cdf(3) - 1.0).abs() < 1e-10);
/// assert!((dist.pmf(1) - 0.25).abs() < 1e-10);
/// assert!((dist.pmf(2) - 0.5).abs() < 1e-10);
/// assert!((dist.pmf(3) - 0.25).abs() < 1e-10);
/// assert!((dist.mean().unwrap() - 2.0).abs() < 1e-10);
/// assert!((dist.variance().unwrap() - 0.5).abs() < 1e-10);
/// assert!((dist.stddev().unwrap() - 0.5f64.sqrt()).abs() < 1e-10);
/// ```
pub struct CustomDiscreteFinite<T> {
    items_map: HashMap<T, usize>,  // (value, index in items_vec)
    items_vec: Vec<(T, f64, f64)>, // (value, probability, cumulative_probability before this value)
}
impl<T> CustomDiscreteFinite<T>
where
    T: Hash + Eq + Copy + PartialOrd,
{
    /// Create a new custom discrete finite distribution.
    /// # Arguments
    /// * `items` - An iterable collection of tuples where each tuple is `(value, probability)`.
    /// # Returns
    /// * A new [CustomDiscreteFinite] distribution.
    /// # Panics
    /// * If any probability is negative.
    /// * If the sum of all probabilities is `0`.
    /// * If any probability cannot be converted to [f64].
    /// * If values of type `T` cannot be compared (needed for sorting).
    /// # Notes
    /// * The probabilities are normalized to sum to `1`, so the input probabilities
    ///   don't have to sum to `1`.
    /// * The order of tuples in `items` doesn't matter since they will be sorted by their values.
    pub fn new<U, V, Z>(items: U) -> Self
    where
        U: IntoIterator<Item = V>,
        V: Borrow<(T, Z)>,
        Z: ToPrimitive + Copy,
    {
        let mut items_map: HashMap<T, usize> = HashMap::new();
        let mut items_vec: Vec<(T, f64, f64)> = Vec::new();
        let mut total_weight = 0.0;

        for (val, prob) in items.into_iter().map(|i| *i.borrow()) {
            let prob = prob
                .to_f64()
                .expect("Probability cannot be converted to f64");
            if prob < 0.0 {
                panic!("Probability must be non-negative.");
            }
            total_weight += prob;
            if let Some(&index) = items_map.get(&val) {
                items_vec[index].1 += prob; // Accumulate probability if value already exists
            } else {
                items_map.insert(val, items_vec.len());
                items_vec.push((val, prob, 0.0));
            }
        }
        if total_weight == 0.0 {
            panic!("Total weight cannot be zero.");
        }

        // sort items_vec by value
        items_vec
            .sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).expect("Values cannot be compared"));

        // update items_map with new indices after sorting,
        // normalize probabilities and calculate cumulative probabilities
        let mut total_prob = 0.0;
        for (i, (val, prob, cum_prob)) in items_vec.iter_mut().enumerate() {
            *items_map.get_mut(val).unwrap() = i;
            *prob /= total_weight; // Normalize probability
            *cum_prob = total_prob;
            total_prob += *prob;
        }

        Self {
            items_map,
            items_vec,
        }
    }

    /// Get the items and their probabilities.
    /// # Returns
    /// * An iterator over tuples of the form `(value, probability)`.
    pub fn items(&self) -> impl Iterator<Item = (T, f64)> {
        self.items_vec.iter().map(|(val, prob, _)| (*val, *prob))
    }
}
impl<T> Distribution<T> for CustomDiscreteFinite<T>
where
    T: ToPrimitive + Copy,
{
    fn cdf<U: ToPrimitive>(&self, x: U) -> f64 {
        let x_f64 = x.to_f64().expect("x cannot be converted to f64");
        if x_f64
            < self
                .items_vec
                .first()
                .unwrap()
                .0
                .to_f64()
                .expect("Value cannot be converted to f64")
        {
            0.0
        } else if x_f64
            >= self
                .items_vec
                .last()
                .unwrap()
                .0
                .to_f64()
                .expect("Value cannot be converted to f64")
        {
            1.0
        } else {
            // Find the index of the first item with value greater than x
            let index = self.items_vec.partition_point(|item| {
                item.0.to_f64().expect("Value cannot be converted to f64") <= x_f64
            });
            // index shouldn't be 0 here since that case is covered by the first branch of this if statement
            // add probability to cumulative probability (since cumulative probability stored is cumulative probability before this value)
            self.items_vec[index - 1].1 + self.items_vec[index - 1].2
        }
    }

    fn mean(&self) -> Option<f64> {
        Some(
            self.items_vec
                .iter()
                .map(|(val, prob, _)| val.to_f64().expect("Cannot convert to f64.") * prob)
                .sum(),
        )
    }

    fn variance(&self) -> Option<f64> {
        let ex2 = self
            .items_vec
            .iter()
            .map(|(val, prob, _)| val.to_f64().expect("Cannot convert to f64.").powi(2) * prob)
            .sum::<f64>();
        Some(ex2 - self.mean()?.powi(2))
    }
}
impl<T> RandDistribution<T> for CustomDiscreteFinite<T>
where
    T: ToPrimitive + Copy,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        let rand_float: f64 = rng.random();
        self.items_vec[self.items_vec.partition_point(|item| item.2 <= rand_float) - 1].0
    }
}
impl<T> DiscreteDistribution<T> for CustomDiscreteFinite<T>
where
    T: ToPrimitive + Copy + Hash + Eq,
{
    fn pmf(&self, x: T) -> f64 {
        match self.items_map.get(&x) {
            Some(val) => self.items_vec[*val].1,
            None => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::assert_float_absolute_eq;
    use rand::rng;

    mod continuous_uniform {
        use super::*;

        #[test]
        fn main() {
            let dist = ContinuousUniform::new(0.0, 1.0);
            assert_float_absolute_eq!(dist.cdf(0.5), 0.5, 1e-10);
            assert_float_absolute_eq!(dist.pdf(0.5), 1.0, 1e-10);
            assert_float_absolute_eq!(dist.mean().unwrap(), 0.5, 1e-10);
            assert_float_absolute_eq!(dist.variance().unwrap(), 1.0 / 12.0, 1e-10);
            assert_float_absolute_eq!(dist.stddev().unwrap(), (1.0 / 12.0f64).sqrt(), 1e-10);
        }

        #[test]
        fn samples() {
            let dist = ContinuousUniform::new(0.0, 1.0);
            for sample in dist.sample_iter(rng()).take(1_000_000) {
                if !(0.0..=1.0).contains(&sample) {
                    panic!("Sample {sample} not in [0, 1]");
                }
            }
        }
    }

    mod discrete_uniform {
        use super::*;

        #[test]
        fn main() {
            let dist = DiscreteUniform::new(1, 3);
            assert_float_absolute_eq!(dist.cdf(2), 2.0 / 3.0, 1e-10);
            assert_float_absolute_eq!(dist.pmf(2), 1.0 / 3.0, 1e-10);
            assert_float_absolute_eq!(dist.mean().unwrap(), 2.0, 1e-10);
            assert_float_absolute_eq!(dist.variance().unwrap(), 2.0 / 3.0, 1e-10);
            assert_float_absolute_eq!(dist.stddev().unwrap(), (2.0 / 3.0f64).sqrt(), 1e-10);
        }

        #[test]
        fn samples() {
            let dist = DiscreteUniform::new(1, 3);
            for sample in dist.sample_iter(rng()).take(1_000_000) {
                if ![1, 2, 3].contains(&sample) {
                    panic!("Sample {sample} not in [1, 2, 3]");
                }
            }
        }
    }

    mod custom_discrete_finite {
        use super::*;

        #[test]
        fn main() {
            let dist = CustomDiscreteFinite::new([(1, 0.25), (2, 0.5), (3, 0.25)]);
            assert_float_absolute_eq!(dist.cdf(1), 0.25, 1e-10);
            assert_float_absolute_eq!(dist.cdf(2), 0.75, 1e-10);
            assert_float_absolute_eq!(dist.cdf(3), 1.0, 1e-10);
            assert_float_absolute_eq!(dist.pmf(1), 0.25, 1e-10);
            assert_float_absolute_eq!(dist.pmf(2), 0.5, 1e-10);
            assert_float_absolute_eq!(dist.pmf(3), 0.25, 1e-10);
            assert_float_absolute_eq!(dist.mean().unwrap(), 2.0, 1e-10);
            assert_float_absolute_eq!(dist.variance().unwrap(), 0.5, 1e-10);
            assert_float_absolute_eq!(dist.stddev().unwrap(), 0.5f64.sqrt(), 1e-10);
        }

        #[test]
        fn samples() {
            let dist = CustomDiscreteFinite::new([(1, 0.25), (2, 0.5), (3, 0.25)]);
            for sample in dist.sample_iter(rng()).take(1_000_000) {
                if ![1, 2, 3].contains(&sample) {
                    panic!("Sample {sample} not in [1, 2, 3]");
                }
            }
        }
    }
}
