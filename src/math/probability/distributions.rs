use rand::distr::Distribution as RandDistribution;

pub trait Distribution<T>: RandDistribution<T> {
    fn cdf(&self, x: T) -> f64;
    fn pdf(&self, x: T) -> f64;
    fn variance(&self) -> Option<f64>;
    fn stddev(&self) -> Option<f64> {
        self.variance().map(|v| v.sqrt())
    }
    fn expectation(&self) -> Option<f64>;
    fn median(&self) -> Option<f64>;
    fn mode(&self) -> Option<f64>;
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
pub struct Uniform {
    min: f64,
    max: f64,
}
pub struct CustomFinite {
    values: Vec<f64>,
    probabilities: Vec<f64>,
}
