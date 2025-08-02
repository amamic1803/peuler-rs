//! Statistics calculations.

use num_traits::{FromPrimitive, ToPrimitive};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

#[cfg_attr(doc, katexit::katexit)]
/// A sample of data points for statistical calculations.
///
/// This struct contains the collection of data points which can be accessed directly
/// (from underlying [Vec]) since it implements the [Deref] trait.
/// # Example
/// ```
/// use peuler::math::statistics::Sample;
///
/// let mut sample = Sample::new([2, 2, 2, 4, 3, 3, 3, 3, 4, 5]);
///
/// assert_eq!(sample[0], 2);
/// sample[0] = 1;
/// assert_eq!(sample[0], 1);
/// assert_eq!(sample.len(), 10);
///
/// assert_eq!(sample.mean().unwrap(), 3.0);
/// assert_eq!(sample.median().unwrap(), 3.0);
/// assert_eq!(sample.mode().unwrap(), 3);
/// assert_eq!(sample.sample_variance().unwrap(), 4.0 / 3.0);
/// assert_eq!(sample.sample_stddev().unwrap(), (4.0_f64 / 3.0).sqrt());
/// assert_eq!(sample.population_variance().unwrap(), 1.2);
/// assert_eq!(sample.population_stddev().unwrap(), 1.2_f64.sqrt());
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Sample<T> {
    data: Vec<T>,
}
impl<T> Sample<T>
where
    T: Copy,
{
    /// Create a new [Sample] from data points.
    /// # Arguments
    /// * `data` - The data points to be included in the sample.
    /// # Returns
    /// * A new [Sample] instance containing the provided data points.
    pub fn new<U, I>(data: U) -> Self
    where
        U: IntoIterator<Item = I>,
        I: Borrow<T>,
    {
        let data = data.into_iter().map(|t| *t.borrow()).collect();
        Self { data }
    }

    /// Calculate arithmetic mean of the sample.
    ///
    /// Arithmetic mean of the sample is defined as:
    /// $$
    ///     \\overline{x} = \\frac{\\sum_{i=1}^{n} x_i}{n}
    /// $$
    /// # Returns
    /// * An [Option] containing the arithmetic mean if the sample is not empty.
    /// # Panics
    /// * If the data points cannot be converted to [f64].
    pub fn mean(&self) -> Option<f64>
    where
        T: ToPrimitive,
    {
        if self.is_empty() {
            return None;
        }
        let sum: f64 = self
            .iter()
            .map(|&x| x.to_f64().expect("Cannot convert to f64."))
            .sum();
        Some(sum / self.len() as f64)
    }

    /// Calculate median of the sample.
    ///
    /// Median of the sample is defined as:
    /// * If the sample has an odd number of elements, it is the middle element.
    /// * If the sample has an even number of elements, it is the average of the two middle elements.
    /// # Returns
    /// * An [Option] containing the median if the sample is not empty.
    /// # Panics
    /// * If the data points cannot be converted to [f64].
    pub fn median(&self) -> Option<f64>
    where
        T: ToPrimitive,
    {
        if self.is_empty() {
            return None;
        }
        let mut sorted = Vec::with_capacity(self.len());
        for &value in &self.data {
            sorted.push(value.to_f64().expect("Cannot convert to f64."));
        }
        sorted.sort_unstable_by(|x, x1| x.partial_cmp(x1).expect("Cannot compare f64 values."));
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            // Even number of elements, average the two middle values
            let left = sorted[mid - 1].to_f64().expect("Cannot convert to f64.");
            let right = sorted[mid].to_f64().expect("Cannot convert to f64.");
            Some((left + right) / 2.0)
        } else {
            // Odd number of elements, return the middle value
            Some(sorted[mid].to_f64().expect("Cannot convert to f64."))
        }
    }

    /// Calculate mode of the sample.
    ///
    /// Mode of the sample is defined as the value that appears most frequently.
    /// # Returns
    /// * An [Option] containing the mode if the sample is not empty.
    /// # Panics
    /// * If the data points cannot be converted to [f64].
    pub fn mode(&self) -> Option<T>
    where
        T: ToPrimitive + FromPrimitive,
    {
        if self.is_empty() {
            return None;
        }
        let mut occurrences = HashMap::new();
        for &value in &self.data {
            *occurrences
                .entry(value.to_f64().expect("Cannot convert to f64.").to_bits())
                .or_insert(0) += 1usize;
        }
        Some(
            T::from_f64(
                occurrences
                    .into_iter()
                    .max_by_key(|&(_, count)| count)
                    .map(|(value, _)| f64::from_bits(value))
                    .unwrap(),
            )
            .unwrap(),
        )
    }

    /// Calculate variance of the sample.
    ///
    /// Variance of the sample from a population is defined as:
    /// $$
    ///   s^2 = \\frac{\\sum_{i=1}^{n} (x_i - \\overline{x})^2}{n - 1}
    /// $$
    /// # Returns
    /// * An [Option] containing the sample variance if the sample has at least 2 points.
    /// # Panics
    /// * If the data points cannot be converted to [f64].
    pub fn sample_variance(&self) -> Option<f64>
    where
        T: ToPrimitive,
    {
        if self.len() < 2 {
            return None; // Variance is undefined for samples with less than 2 points.
        }
        let mean = self.mean()?;
        let sum: f64 = self
            .iter()
            .map(|&x| {
                let diff = x.to_f64().expect("Cannot convert to f64.") - mean;
                diff * diff
            })
            .sum();
        Some(sum / (self.len() as f64 - 1.0))
    }

    /// Calculate standard deviation of the sample.
    ///
    /// Standard deviation of the sample from a population is defined as:
    /// $$
    ///     s = \\sqrt{\\frac{\\sum_{i=1}^{n} (x_i - \\overline{x})^2}{n - 1}}
    /// $$
    /// # Returns
    /// * An [Option] containing the sample standard deviation if the sample has at least 2 points.
    /// # Panics
    /// * If the data points cannot be converted to [f64].
    pub fn sample_stddev(&self) -> Option<f64>
    where
        T: ToPrimitive,
    {
        self.sample_variance().map(|v| v.sqrt())
    }

    /// Calculate variance of the population.
    ///
    /// Variance of the population is defined as:
    /// $$
    ///   \\sigma^2 = \\frac{\\sum_{i=1}^{n} (x_i - \\overline{x})^2}{n}
    /// $$
    ///
    /// Use this when the sample represents the entire population.
    /// # Returns
    /// * An [Option] containing the population variance if the sample is not empty.
    /// # Panics
    /// * If the data points cannot be converted to [f64].
    pub fn population_variance(&self) -> Option<f64>
    where
        T: ToPrimitive,
    {
        let mean = self.mean()?;
        let sum: f64 = self
            .iter()
            .map(|&x| {
                let diff = x.to_f64().expect("Cannot convert to f64.") - mean;
                diff * diff
            })
            .sum();
        Some(sum / self.len() as f64)
    }

    /// Calculate standard deviation of the population.
    ///
    /// Standard deviation of the population is defined as:
    /// $$
    ///   \\sigma = \\sqrt{\\frac{\\sum_{i=1}^{n} (x_i - \\overline{x})^2}{n}}
    /// $$
    ///
    /// Use this when the sample represents the entire population.
    /// # Returns
    /// * An [Option] containing the population standard deviation if the sample is not empty.
    /// # Panics
    /// * If the data points cannot be converted to [f64].
    pub fn population_stddev(&self) -> Option<f64>
    where
        T: ToPrimitive,
    {
        self.population_variance().map(|v| v.sqrt())
    }
}
impl<T> Default for Sample<T>
where
    T: Copy,
{
    fn default() -> Self {
        Self::new::<_, T>([])
    }
}
impl<T> Deref for Sample<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> DerefMut for Sample<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
