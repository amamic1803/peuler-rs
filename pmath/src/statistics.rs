//! Statistics calculations.

use num_traits::{FromPrimitive, ToPrimitive};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Deref;

// this is imported for usage in the doc comments
#[allow(unused_imports)]
use std::ops::DerefMut;

#[cfg_attr(doc, katexit::katexit)]
/// A sample of data points for statistical calculations.
///
/// This struct contains the collection of data points which can be accessed directly
/// (from the underlying [Vec]) since it implements the [Deref] trait.
///
/// [DerefMut] is not implemented since this
/// uses _Welford's online algorithm_ for calculating variance and standard deviation.
/// There are a few methods (similar to methods from [Vec])
/// to add or remove data points from the sample.
/// # Example
/// ```
/// use pmath::statistics::Sample;
///
/// let mut sample = Sample::from_values([2, 2, 2, 4, 3, 3, 3, 3, 4, 5]);
///
/// assert_eq!(sample[0], 2);
/// sample.remove(0);
/// sample.insert(0, 1);
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
#[derive(Clone, PartialEq)]
pub struct Sample<T> {
    data: Vec<T>,
    mean: Option<f64>,
    m2: Option<f64>,
}
impl<T> Sample<T>
where
    T: Copy + FromPrimitive + ToPrimitive,
{
    /// Create a new empty [Sample].
    /// # Returns
    /// * A new [Sample] instance with no data points.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            mean: None,
            m2: None,
        }
    }

    /// Create a new [Sample] from data points.
    /// # Arguments
    /// * `data` - The data points to be included in the sample.
    /// # Returns
    /// * A new [Sample] instance containing the provided data points.
    /// # Panics
    /// * If the data points cannot be converted to [f64].
    pub fn from_values<U, I>(data: U) -> Self
    where
        U: IntoIterator<Item = I>,
        I: Borrow<T>,
    {
        let mut sample = Self::new();
        for value in data.into_iter().map(|t| *t.borrow()) {
            sample.push(value);
        }
        sample
    }

    /// Add a data point to the end of the sample.
    /// # Arguments
    /// * `value` - The data point to be added.
    /// # Panics
    /// * If the data point cannot be converted to [f64].
    pub fn push(&mut self, value: T) {
        self.insert(self.len(), value);
    }

    /// Remove the last data point from the sample and return it.
    /// # Returns
    /// * An [Option] containing the removed data point if the sample is not empty.
    /// # Panics
    /// * If the data point cannot be converted to [f64].
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(self.len() - 1))
        }
    }

    /// Insert a data point at the specified index in the sample.
    /// # Arguments
    /// * `index` - The index at which to insert the data point.
    /// * `element` - The data point to be inserted.
    /// # Panics
    /// * If the index is out of bounds.
    /// * If the data point cannot be converted to [f64].
    pub fn insert(&mut self, index: usize, element: T) {
        self.data.insert(index, element);

        let x = element.to_f64().expect("Cannot convert to f64.");

        let delta = x - self.mean.unwrap_or(0.0);
        self.mean = Some(self.mean.unwrap_or(0.0) + delta / (self.len() as f64));

        self.m2 = Some(self.m2.unwrap_or(0.0) + delta * (x - self.mean.unwrap()));
    }

    /// Remove the data point at the specified index from the sample and return it.
    /// # Arguments
    /// * `index` - The index of the data point to be removed.
    /// # Returns
    /// * The removed data point.
    /// # Panics
    /// * If the index is out of bounds.
    /// * If the data point cannot be converted to [f64].
    pub fn remove(&mut self, index: usize) -> T {
        let value = self.data.remove(index);
        let x = value.to_f64().expect("Cannot convert to f64.");
        if self.is_empty() {
            self.mean = None;
            self.m2 = None;
        } else {
            let delta = x - self.mean.unwrap();
            self.mean = Some(self.mean.unwrap() - (delta / (self.len() as f64)));
            self.m2 = Some(self.m2.unwrap() - delta * (x - self.mean.unwrap()));
        }
        value
    }

    /// Clear all data points from the sample.
    pub fn clear(&mut self) {
        self.data.clear();
        self.mean = None;
        self.m2 = None;
    }

    /// Calculate arithmetic mean of the sample.
    ///
    /// Arithmetic mean of the sample is defined as:
    /// $$
    ///     \\overline{x} = \\frac{\\sum_{i=1}^{n} x_i}{n}
    /// $$
    /// # Returns
    /// * An [Option] containing the arithmetic mean if the sample is not empty.
    pub fn mean(&self) -> Option<f64> {
        self.mean
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
    /// * If the data points cannot be compared as [f64] values. For example, if the data points
    ///   contain [f64::NAN] values.
    pub fn median(&self) -> Option<f64> {
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

    /// Calculate the mode of the sample.
    ///
    /// Mode of the sample is defined as the value that appears most frequently.
    /// If there are multiple values with the same highest frequency,
    /// the last one in the sample is returned.
    /// # Returns
    /// * An [Option] containing the mode if the sample is not empty.
    /// # Panics
    /// * If the data points cannot be converted to [f64].
    pub fn mode(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let mut occurrences = HashMap::new();
        for &value in &self.data {
            *occurrences
                .entry(value.to_f64().expect("Cannot convert to f64.").to_bits())
                .or_insert(0) += 1usize;
        }
        let highest_frequency = *occurrences.values().max().unwrap();
        for v in self.iter().rev() {
            if let Some(&count) =
                occurrences.get(&v.to_f64().expect("Cannot convert to f64.").to_bits())
                && count == highest_frequency
            {
                return Some(*v);
            }
        }
        unreachable!("One of the values must be the mode.");
    }

    /// Calculate variance of the sample.
    ///
    /// Variance of the sample from a population is defined as:
    /// $$
    ///   s^2 = \\frac{\\sum_{i=1}^{n} (x_i - \\overline{x})^2}{n - 1}
    /// $$
    /// # Returns
    /// * An [Option] containing the sample variance if the sample has at least 2 points.
    pub fn sample_variance(&self) -> Option<f64> {
        if self.len() < 2 {
            None
        } else {
            Some(self.m2.unwrap() / (self.len() as f64 - 1.0))
        }
    }

    /// Calculate standard deviation of the sample.
    ///
    /// Standard deviation of the sample from a population is defined as:
    /// $$
    ///     s = \\sqrt{\\frac{\\sum_{i=1}^{n} (x_i - \\overline{x})^2}{n - 1}}
    /// $$
    /// # Returns
    /// * An [Option] containing the sample standard deviation if the sample has at least 2 points.
    pub fn sample_stddev(&self) -> Option<f64> {
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
    pub fn population_variance(&self) -> Option<f64> {
        self.m2.map(|m2| m2 / self.len() as f64)
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
    pub fn population_stddev(&self) -> Option<f64> {
        self.population_variance().map(|v| v.sqrt())
    }
}
impl<T> Default for Sample<T>
where
    T: Copy + FromPrimitive + ToPrimitive,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Deref for Sample<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
