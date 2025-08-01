use rand::distr::Distribution as RandDistribution;

pub trait Distribution<T>: RandDistribution<T> {
    fn cdf(&self, x: T) -> f64;
    fn pdf(&self, x: T) -> f64;
    fn variance(&self) -> Option<f64>;
    fn stddev(&self) -> Option<f64> {
        self.variance().map(|v| v.sqrt())
    }
    fn expectation(&self) -> Option<f64>;
}
