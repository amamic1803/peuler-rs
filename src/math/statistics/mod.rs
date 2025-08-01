use num_traits::ToPrimitive;
use std::borrow::Borrow;

pub mod distributions;

pub struct Sample<T> {
    data: Vec<T>,
}
impl<T> Sample<T>
where
    T: Copy,
{
    pub fn new<U, I>(data: U) -> Self
    where
        U: IntoIterator<Item = I>,
        I: Borrow<T>,
    {
        let data = data.into_iter().map(|t| *t.borrow()).collect();
        Self { data }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    pub fn mean(&self) -> Option<f64>
    where
        T: ToPrimitive,
    {
        if self.is_empty() {
            return None;
        }
        let sum: f64 = self
            .data
            .iter()
            .map(|&x| x.to_f64().expect("Cannot convert to f64."))
            .sum();
        Some(sum / self.data.len() as f64)
    }

    pub fn sample_variance(&self) -> Option<f64>
    where
        T: ToPrimitive,
    {
        let mean = self.mean()?;
        let sum: f64 = self
            .data
            .iter()
            .map(|&x| {
                let diff = x.to_f64().expect("Cannot convert to f64.") - mean;
                diff * diff
            })
            .sum();
        Some(sum / (self.data.len() as f64 - 1.0))
    }

    pub fn sample_stddev(&self) -> Option<f64>
    where
        T: ToPrimitive,
    {
        Some(self.sample_variance()?.sqrt())
    }

    pub fn population_variance(&self) -> Option<f64>
    where
        T: ToPrimitive,
    {
        let mean = self.mean()?;
        let sum: f64 = self
            .data
            .iter()
            .map(|&x| {
                let diff = x.to_f64().expect("Cannot convert to f64.") - mean;
                diff * diff
            })
            .sum();
        Some(sum / self.data.len() as f64)
    }

    pub fn population_stddev(&self) -> Option<f64>
    where
        T: ToPrimitive,
    {
        Some(self.population_variance()?.sqrt())
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
