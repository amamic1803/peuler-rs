//! Root finding methods.

use num_traits::ToPrimitive;

/// Newton's method for finding the root of a function.
///
/// If the function does not converge to a zero, this might run indefinitely.
/// It is recommended to use this method only for functions that are known to converge.
/// # Arguments
/// * `x0` - The initial guess.
/// * `precision` - The precision of the answer (the error will be less than this).
/// * `function` - The function to find the root of.
/// * `derivative` - The derivative of the function.
/// # Returns
/// * [Some] with the root of the function if it converges to a root within the given precision,
///   or [None] if the value of the derivative is `0` at any of the evaluated points.
/// # Panics
/// * If `x0` cannot be converted to [f64].
/// # Example
/// ```
/// use pmath::analysis::root::newtons_method;
///
/// // f(x) = x^2 - 2
/// // The root of f(x) is the square root of 2.
/// let x0 = 1.0;
/// let precision = 1e-10;
/// let function = |x| x * x - 2.0;
/// let derivative = |x| 2.0 * x;
/// let calculated_root = newtons_method(x0, precision, function, derivative).unwrap();
/// assert!((calculated_root - 2.0_f64.sqrt()).abs() < precision);
/// ```
pub fn newtons_method<T, F, D>(x0: T, precision: f64, function: F, derivative: D) -> Option<f64>
where
    T: ToPrimitive,
    F: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    let mut x = x0.to_f64().expect("Cannot convert x0 to f64.");
    let mut prev_x = f64::NEG_INFINITY;

    while (x - prev_x).abs() > precision {
        prev_x = x;
        let derivative_value = derivative(prev_x);
        if derivative_value == 0.0 {
            return None; // derivative is zero, cannot proceed
        }
        x = prev_x - function(prev_x) / derivative_value;
    }

    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newtons_method_primitive_types() {
        let precision = 1e-10;
        let function = |x| x * x - 4.0;
        let derivative = |x| 2.0 * x;
        let root = 2.0;

        // unsigned types
        assert!((newtons_method(1u8, precision, function, derivative).unwrap() - root).abs() < precision);
        assert!((newtons_method(1u16, precision, function, derivative).unwrap() - root).abs() < precision);
        assert!((newtons_method(1u32, precision, function, derivative).unwrap() - root).abs() < precision);
        assert!((newtons_method(1u64, precision, function, derivative).unwrap() - root).abs() < precision);
        assert!((newtons_method(1u128, precision, function, derivative).unwrap() - root).abs() < precision);
        assert!((newtons_method(1usize, precision, function, derivative).unwrap() - root).abs() < precision);

        // signed types
        assert!((newtons_method(1i8, precision, function, derivative).unwrap() - root).abs() < precision);
        assert!((newtons_method(1i16, precision, function, derivative).unwrap() - root).abs() < precision);
        assert!((newtons_method(1i32, precision, function, derivative).unwrap() - root).abs() < precision);
        assert!((newtons_method(1i64, precision, function, derivative).unwrap() - root).abs() < precision);
        assert!((newtons_method(1i128, precision, function, derivative).unwrap() - root).abs() < precision);
        assert!((newtons_method(1isize, precision, function, derivative).unwrap() - root).abs() < precision);

        // float types
        assert!((newtons_method(1.0f32, precision, function, derivative).unwrap() - root).abs() < precision);
        assert!((newtons_method(1.0f64, precision, function, derivative).unwrap() - root).abs() < precision);
    }

    #[test]
    fn newtons_method_verify() {
        let precision = 1e-10;
        let function = |x| x * x - 4.0;
        let derivative = |x| 2.0 * x;
        let root = 2.0;
        assert!((newtons_method(1.0, precision, function, derivative).unwrap() - root).abs() < precision);

        let function = |x| x;
        let derivative = |_| 0.0;
        assert!(newtons_method(1.0, precision, function, derivative).is_none());
    }
}
