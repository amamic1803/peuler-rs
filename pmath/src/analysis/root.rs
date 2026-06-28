use num_traits::ToPrimitive;

/// Newton's method for finding the zero of a function.
///
/// If the function does not converge to a zero, this might run indefinitely.
/// It is recommended to use this method only for functions that are known to converge.
/// # Arguments
/// * `x0` - The initial guess.
/// * `precision` - The precision of the answer (the error will be less than this).
/// * `function` - The function to find the zero of.
/// * `derivative` - The derivative of the function.
/// # Returns
/// * [Some] with the zero of the function if it converges to a zero within the given precision,
///   or [None] if the value of the derivative is `0` at some of the evaluated points.
/// # Panics
/// * If `x0` cannot be converted to [f64].
/// # Example
/// ```
/// use pmath::newtons_method;
///
/// // f(x) = x^2 - 2
/// // The zero of f(x) is the square root of 2.
/// let x0 = 1.0;
/// let precision = 1e-10;
/// let function = |x| x * x - 2.0;
/// let derivative = |x| 2.0 * x;
/// let calculated_zero = newtons_method(x0, precision, function, derivative).unwrap();
/// assert!((calculated_zero - 2.0_f64.sqrt()).abs() < precision);
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
