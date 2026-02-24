//! Statistical calculations.

use num_traits::{FromPrimitive, ToPrimitive};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
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
/// assert_eq!(sample.variance().unwrap(), 4.0 / 3.0);
/// assert_eq!(sample.stddev().unwrap(), (4.0_f64 / 3.0).sqrt());
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
            let left = sorted[mid - 1];
            let right = sorted[mid];
            Some((left + right) / 2.0)
        } else {
            // Odd number of elements, return the middle value
            Some(sorted[mid])
        }
    }

    /// Calculate the mode of the sample.
    ///
    /// Mode of the sample is defined as the value that appears most frequently.
    /// If there are multiple values with the same highest frequency,
    /// the last one in the sample is returned.
    /// # Returns
    /// * An [Option] containing the mode if the sample is not empty.
    pub fn mode(&self) -> Option<T>
    where
        T: Eq + Hash,
    {
        if self.is_empty() {
            return None;
        }
        let mut occurrences = HashMap::new();
        for &value in &self.data {
            *occurrences.entry(value).or_insert(0) += 1usize;
        }
        let highest_frequency = *occurrences.values().max().unwrap();
        for v in self.iter().rev() {
            if let Some(&count) = occurrences.get(v)
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
    pub fn variance(&self) -> Option<f64> {
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
    pub fn stddev(&self) -> Option<f64> {
        self.variance().map(|v| v.sqrt())
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::{assert_f64_near, assert_float_absolute_eq};

    #[test]
    fn sample_primitive_types() {
        //! Test the [Sample] struct with primitive types.

        // unsigned types
        let sample: Sample<u8> = Sample::from_values([1, 2, 3, 4, 5]);
        assert_f64_near!(sample.mean().unwrap(), 3.0);
        assert_f64_near!(sample.median().unwrap(), 3.0);
        let sample: Sample<u16> = Sample::from_values([1, 2, 3, 4, 5]);
        assert_f64_near!(sample.mean().unwrap(), 3.0);
        assert_f64_near!(sample.median().unwrap(), 3.0);
        let sample: Sample<u32> = Sample::from_values([1, 2, 3, 4, 5]);
        assert_f64_near!(sample.mean().unwrap(), 3.0);
        assert_f64_near!(sample.median().unwrap(), 3.0);
        let sample: Sample<u64> = Sample::from_values([1, 2, 3, 4, 5]);
        assert_f64_near!(sample.mean().unwrap(), 3.0);
        assert_f64_near!(sample.median().unwrap(), 3.0);
        let sample: Sample<u128> = Sample::from_values([1, 2, 3, 4, 5]);
        assert_f64_near!(sample.mean().unwrap(), 3.0);
        assert_f64_near!(sample.median().unwrap(), 3.0);
        let sample: Sample<usize> = Sample::from_values([1, 2, 3, 4, 5]);
        assert_f64_near!(sample.mean().unwrap(), 3.0);
        assert_f64_near!(sample.median().unwrap(), 3.0);

        // signed types
        let sample: Sample<i8> = Sample::from_values([-1, 0, 1, 2, 3]);
        assert_f64_near!(sample.mean().unwrap(), 1.0);
        assert_f64_near!(sample.median().unwrap(), 1.0);
        let sample: Sample<i16> = Sample::from_values([-1, 0, 1, 2, 3]);
        assert_f64_near!(sample.mean().unwrap(), 1.0);
        assert_f64_near!(sample.median().unwrap(), 1.0);
        let sample: Sample<i32> = Sample::from_values([-1, 0, 1, 2, 3]);
        assert_f64_near!(sample.mean().unwrap(), 1.0);
        assert_f64_near!(sample.median().unwrap(), 1.0);
        let sample: Sample<i64> = Sample::from_values([-1, 0, 1, 2, 3]);
        assert_f64_near!(sample.mean().unwrap(), 1.0);
        assert_f64_near!(sample.median().unwrap(), 1.0);
        let sample: Sample<i128> = Sample::from_values([-1, 0, 1, 2, 3]);
        assert_f64_near!(sample.mean().unwrap(), 1.0);
        assert_f64_near!(sample.median().unwrap(), 1.0);
        let sample: Sample<isize> = Sample::from_values([-1, 0, 1, 2, 3]);
        assert_f64_near!(sample.mean().unwrap(), 1.0);
        assert_f64_near!(sample.median().unwrap(), 1.0);
    }

    #[test]
    fn sample_new() {
        //! Test the [Sample::new] constructor.

        let sample: Sample<i32> = Sample::new();
        assert!(sample.is_empty());
    }

    #[test]
    fn sample_default() {
        //! Test the [Default] implementation for [Sample].

        let default_sample: Sample<i32> = Sample::default();
        assert!(default_sample.is_empty());
    }

    #[test]
    fn sample_from_values() {
        //! Test the [Sample::from_values] constructor.

        let sample = Sample::from_values([1, 2, 3, 4, 5]);
        assert_eq!(sample.len(), 5);
        assert_eq!(sample[0], 1);
        assert_eq!(sample[1], 2);
        assert_eq!(sample[2], 3);
        assert_eq!(sample[3], 4);
        assert_eq!(sample[4], 5);

        let v: Vec<i32> = vec![];
        let sample = Sample::from_values(v);
        assert!(sample.is_empty());
    }

    #[test]
    fn sample_push() {
        //! Test the [Sample::push] method.

        let mut sample = Sample::new();
        sample.push(1);
        sample.push(2);
        sample.push(3);
        assert_eq!(sample.len(), 3);
        assert_eq!(sample[0], 1);
        assert_eq!(sample[1], 2);
        assert_eq!(sample[2], 3);
    }

    #[test]
    fn sample_pop() {
        //! Test the [Sample::pop] method.

        let mut sample = Sample::from_values([1, 2, 3]);
        assert_eq!(sample.pop(), Some(3));
        assert_eq!(sample.pop(), Some(2));
        assert_eq!(sample.pop(), Some(1));
        assert_eq!(sample.pop(), None);
        assert_eq!(sample.len(), 0);
    }

    #[test]
    fn sample_insert() {
        //! Test the [Sample::insert] method.

        let mut sample = Sample::from_values([1, 2, 4]);
        sample.insert(2, 3); // in the middle
        sample.insert(0, 0); // at the beginning
        sample.insert(5, 5); // at the end
        assert_eq!(sample.len(), 6);
        assert_eq!(sample[0], 0);
        assert_eq!(sample[1], 1);
        assert_eq!(sample[2], 2);
        assert_eq!(sample[3], 3);
        assert_eq!(sample[4], 4);
        assert_eq!(sample[5], 5);
    }

    #[test]
    #[should_panic]
    fn sample_insert_out_of_bounds() {
        //! Test that the [Sample::insert] method panics when the index is out of bounds.

        let mut sample = Sample::from_values([1, 2, 3]);
        sample.insert(4, 4);
    }

    #[test]
    fn sample_remove() {
        //! Test the [Sample::remove] method.

        let mut sample = Sample::from_values([1, 2, 3, 4, 5]);
        assert_eq!(sample.remove(2), 3); // in the middle
        assert_eq!(sample.remove(0), 1); // at the beginning
        assert_eq!(sample.remove(2), 5); // at the end
        assert_eq!(sample.len(), 2);
        assert_eq!(sample[0], 2);
        assert_eq!(sample[1], 4);
    }

    #[test]
    #[should_panic]
    fn sample_remove_out_of_bounds() {
        //! Test that the [Sample::remove] method panics when the index is out of bounds.

        let mut sample = Sample::from_values([1, 2, 3]);
        sample.remove(3);
    }

    #[test]
    fn sample_clear() {
        //! Test the [Sample::clear] method.

        let mut sample = Sample::new();
        sample.push(1);
        sample.push(2);
        sample.push(3);
        assert_eq!(sample.len(), 3);
        sample.clear();
        assert!(sample.is_empty());
        assert_eq!(sample.mean(), None);
        assert_eq!(sample.median(), None);
        assert_eq!(sample.mode(), None);
        assert_eq!(sample.variance(), None);
        assert_eq!(sample.stddev(), None);
        assert_eq!(sample.population_variance(), None);
        assert_eq!(sample.population_stddev(), None);
    }

    #[test]
    fn sample_mean() {
        //! Test the [Sample::mean] method.

        let mut sample = Sample::from_values([1, 2, 3, 4, 5]);
        assert_f64_near!(sample.mean().unwrap(), 3.0);
        sample.insert(0, 0);
        assert_f64_near!(sample.mean().unwrap(), 2.5);
        sample.remove(5);
        assert_f64_near!(sample.mean().unwrap(), 2.0);
        sample.clear();
        assert_eq!(sample.mean(), None);
    }

    #[test]
    fn sample_median() {
        //! Test the [Sample::median] method.

        let mut sample = Sample::from_values([1, 2, 3, 4, 5]);
        assert_f64_near!(sample.median().unwrap(), 3.0);
        sample.insert(0, 0);
        assert_f64_near!(sample.median().unwrap(), 2.5);
        sample.remove(5);
        assert_f64_near!(sample.median().unwrap(), 2.0);
        sample.clear();
        assert_eq!(sample.median(), None);
    }

    #[test]
    #[should_panic]
    fn sample_median_nan() {
        //! Test that the [Sample::median] method panics when the data points
        //! cannot be compared as [f64] values.

        let sample = Sample::from_values([1.0, 2.0, f64::NAN]);
        sample.median();
    }

    #[test]
    fn sample_mode() {
        //! Test the [Sample::mode] method.

        let mut sample = Sample::from_values([1, 2, 3, 4, 5]);
        assert_eq!(sample.mode().unwrap(), 5);
        sample.insert(0, 1);
        assert_eq!(sample.mode().unwrap(), 1);
        sample.insert(0, 2);
        assert_eq!(sample.mode().unwrap(), 2);
        sample.remove(0);
        assert_eq!(sample.mode().unwrap(), 1);
        sample.clear();
        assert_eq!(sample.mode(), None);
    }

    #[test]
    fn sample_variance() {
        //! Test the [Sample::variance] method.

        let mut sample = Sample::from_values([1, 2, 3, 4, 5]);
        assert_f64_near!(sample.variance().unwrap(), 2.5);
        sample.insert(0, 0);
        assert_f64_near!(sample.variance().unwrap(), 3.5);
        sample.remove(5);
        assert_f64_near!(sample.variance().unwrap(), 2.5);
        sample.clear();
        assert_eq!(sample.variance(), None);
    }

    #[test]
    fn sample_stddev() {
        //! Test the [Sample::stddev] method.

        let mut sample = Sample::from_values([1, 2, 3, 4, 5]);
        assert_f64_near!(sample.stddev().unwrap(), 2.5_f64.sqrt());
        sample.insert(0, 0);
        assert_f64_near!(sample.stddev().unwrap(), 3.5_f64.sqrt());
        sample.remove(5);
        assert_f64_near!(sample.stddev().unwrap(), 2.5_f64.sqrt());
        sample.clear();
        assert_eq!(sample.stddev(), None);
    }

    #[test]
    fn sample_population_variance() {
        //! Test the [Sample::population_variance] method.

        let mut sample = Sample::from_values([1, 2, 3, 4, 5]);
        assert_f64_near!(sample.population_variance().unwrap(), 2.0);
        sample.insert(0, 0);
        assert_float_absolute_eq!(sample.population_variance().unwrap(), 2.916666666666667);
        sample.remove(5);
        assert_f64_near!(sample.population_variance().unwrap(), 2.0);
        sample.clear();
        assert_eq!(sample.population_variance(), None);
    }

    #[test]
    fn sample_population_stddev() {
        //! Test the [Sample::population_stddev] method.

        let mut sample = Sample::from_values([1, 2, 3, 4, 5]);
        assert_f64_near!(sample.population_stddev().unwrap(), 2.0_f64.sqrt());
        sample.insert(0, 0);
        assert_float_absolute_eq!(
            sample.population_stddev().unwrap(),
            2.916666666666667_f64.sqrt()
        );
        sample.remove(5);
        assert_f64_near!(sample.population_stddev().unwrap(), 2.0_f64.sqrt());
        sample.clear();
        assert_eq!(sample.population_stddev(), None);
    }
}
