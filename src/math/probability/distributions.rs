//! Probability distributions.

use num_traits::ToPrimitive;
use rand::distr::Distribution as RandDistribution;

/// A trait representing a generic probability distribution.
pub trait Distribution<T>: RandDistribution<T> {
    /// Cumulative distribution function (CDF).
    /// # Arguments
    /// * `x` - The value at which to evaluate the CDF.
    /// # Returns
    /// * The value of the CDF at `x`.
    fn cdf<U: ToPrimitive>(&self, x: U) -> f64;

    /// Inverse cumulative distribution function (quantile function)
    fn quantile(&self, p: f64) -> Option<f64>;

    /// Mean of the distribution.
    ///
    /// Also known as the expected value or expectation.
    /// # Returns
    /// * An [Option] with the mean of the distribution if it is defined.
    fn mean(&self) -> Option<f64>;

    /// Variance of the distribution.
    /// # Returns
    /// * An [Option] with the variance of the distribution if it is defined.
    fn variance(&self) -> Option<f64>;

    /// Standard deviation of the distribution.
    /// # Returns
    /// * An [Option] with the standard deviation of the distribution if it is defined.
    fn stddev(&self) -> Option<f64> {
        self.variance().map(|v| v.sqrt())
    }

    /// Median of the distribution.
    /// # Returns
    /// * An [Option] with the median of the distribution if it is defined.
    fn median(&self) -> Option<f64> {
        self.quantile(0.5)
    }

    /// Mode of the distribution.
    /// # Returns
    /// * An [Option] with the mode of the distribution if it is defined.
    fn mode(&self) -> Option<f64>;
}

/// A trait representing a discrete probability distribution.
pub trait DiscreteDistribution<T>: Distribution<T> {
    /// Probability mass function (PMF)
    fn pmf(&self, x: T) -> f64;
}

/// A trait representing a continuous probability distribution.
pub trait ContinuousDistribution<T>: Distribution<T> {
    /// Probability density function (PDF)
    fn pdf(&self, x: T) -> f64;
}

pub struct ContinuousUniform {
    min: f64,
    max: f64,
}

pub struct DiscreteUniform {
    min: u64,
    max: u64,
}

pub struct Binomial {
    n: u64,
    p: f64,
}

pub struct Geometric {
    p: f64,
}

pub struct Poisson {
    lambda: f64,
}

pub struct CustomFinite {
    values: Vec<f64>,
    probabilities: Vec<f64>,
}
