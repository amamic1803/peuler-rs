//! A collection of solutions to the problems from [*Project Euler*](https://projecteuler.net).
//! # Example
//! ```
//! use peuler::{PEuler, ProjectEuler};
//!
//! let peuler = PEuler::new();
//! assert_eq!(peuler.solve(1).unwrap(), "233168");
//! ```

use std::cmp::Ordering;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::time::Duration;

pub mod problems;

/// An enum representing errors that can occur in this crate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
    /// The requested problem is not available.
    UnavailableProblem,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnavailableProblem => write!(f, "The requested problem is not available."),
        }
    }
}
impl StdError for Error {}

/// A trait representing the [*Project Euler*](https://projecteuler.net).
pub trait ProjectEuler: Send + Sync {
    /// Get all available problems.
    /// # Returns
    /// * An iterator over the problems, sorted by their identifier in ascending order.
    fn problems<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Problem> + 'a>;

    /// Get the problem with the specified identifier.
    /// # Arguments
    /// * `id` - The identifier of the problem to retrieve.
    /// # Returns
    /// * A reference to the specified problem or an [Error].
    /// # Errors
    /// * [Error::UnavailableProblem] - If the specified problem is not available.
    fn problem(&self, id: usize) -> Result<&dyn Problem, Error> {
        for iter_problem in self.problems() {
            match iter_problem.id().cmp(&id) {
                Ordering::Less => {}
                Ordering::Equal => return Ok(iter_problem),
                Ordering::Greater => break,
            }
        }
        Err(Error::UnavailableProblem)
    }

    /// Solve the specified problem.
    /// # Arguments
    /// * `id` - The identifier of the problem to solve.
    /// # Returns
    /// * The solution to the problem or the [Error].
    /// # Errors
    /// * [Error::UnavailableProblem] - If the specified problem is not available.
    fn solve(&self, id: usize) -> Result<String, Error> {
        Ok(self.problem(id)?.solve())
    }

    /// Solve the specified problem and measure the elapsed time.
    /// # Arguments
    /// * `id` - The identifier of the problem to run.
    /// # Returns
    /// * The solution to the problem and the elapsed time or the [Error].
    /// # Errors
    /// * [Error::UnavailableProblem] - If the specified problem is not available.
    fn benchmark(&self, id: usize) -> Result<(String, Duration), Error> {
        Ok(self.problem(id)?.benchmark())
    }
}

/// A trait representing the [*Project Euler*](https://projecteuler.net) problem.
pub trait Problem: Send + Sync + Solution {
    /// The identifier of the problem.
    /// # Returns
    /// * An integer greater than `0` representing the problem's ID.
    fn id(&self) -> usize;

    /// The title of the problem.
    /// # Returns
    /// * The title of the problem.
    fn title(&self) -> &str;
}

/// A trait representing the [*Project Euler*](https://projecteuler.net) problem's solution.
pub trait Solution: Send + Sync {
    /// Solve the problem.
    /// # Returns
    /// * The solution to the problem.
    fn solve(&self) -> String;

    /// Solve the problem and measure the elapsed time.
    /// # Returns
    /// * The solution to the problem and the elapsed time.
    fn benchmark(&self) -> (String, Duration) {
        let result;
        let elapsed;

        #[cfg(not(all(
            target_arch = "wasm32",
            target_vendor = "unknown",
            target_os = "unknown"
        )))]
        {
            let instant = std::time::Instant::now();
            result = self.solve();
            elapsed = instant.elapsed();
        }
        #[cfg(all(
            target_arch = "wasm32",
            target_vendor = "unknown",
            target_os = "unknown"
        ))]
        {
            use wasm_bindgen::prelude::*;

            let global_obj = js_sys::global();
            let performance = js_sys::Reflect::get(&global_obj, &JsValue::from_str("performance"))
                .unwrap()
                .dyn_into::<web_sys::Performance>()
                .unwrap();

            let instant = performance.now();
            result = self.solve();
            elapsed = Duration::from_secs_f64((performance.now() - instant) / 1000.0);
        }

        (result, elapsed)
    }
}

/// A structure representing the [*Project Euler*](https://projecteuler.net).
pub struct PEuler {
    problems: Vec<Box<dyn Problem>>,
}
impl PEuler {
    /// Create a new [PEuler] instance.
    /// # Returns
    /// * The new [PEuler] instance with all available problems initialized.
    pub fn new() -> Self {
        let mut new_obj = Self {
            problems: vec![
                Box::new(problems::Problem0001::new()),
                Box::new(problems::Problem0002::new()),
                Box::new(problems::Problem0003::new()),
                Box::new(problems::Problem0004::new()),
                Box::new(problems::Problem0005::new()),
                Box::new(problems::Problem0006::new()),
                Box::new(problems::Problem0007::new()),
                Box::new(problems::Problem0008::new()),
                Box::new(problems::Problem0009::new()),
                Box::new(problems::Problem0010::new()),
                Box::new(problems::Problem0011::new()),
                Box::new(problems::Problem0012::new()),
                Box::new(problems::Problem0013::new()),
                Box::new(problems::Problem0014::new()),
                Box::new(problems::Problem0015::new()),
                Box::new(problems::Problem0016::new()),
                Box::new(problems::Problem0017::new()),
                Box::new(problems::Problem0018::new()),
                Box::new(problems::Problem0019::new()),
                Box::new(problems::Problem0020::new()),
                Box::new(problems::Problem0021::new()),
                Box::new(problems::Problem0022::new()),
                Box::new(problems::Problem0023::new()),
                Box::new(problems::Problem0024::new()),
                Box::new(problems::Problem0025::new()),
                Box::new(problems::Problem0026::new()),
                Box::new(problems::Problem0027::new()),
                Box::new(problems::Problem0028::new()),
                Box::new(problems::Problem0029::new()),
                Box::new(problems::Problem0030::new()),
                Box::new(problems::Problem0031::new()),
                Box::new(problems::Problem0032::new()),
                Box::new(problems::Problem0033::new()),
                Box::new(problems::Problem0034::new()),
                Box::new(problems::Problem0035::new()),
                Box::new(problems::Problem0036::new()),
                Box::new(problems::Problem0037::new()),
                Box::new(problems::Problem0038::new()),
                Box::new(problems::Problem0039::new()),
                Box::new(problems::Problem0040::new()),
                Box::new(problems::Problem0041::new()),
                Box::new(problems::Problem0042::new()),
                Box::new(problems::Problem0043::new()),
                Box::new(problems::Problem0044::new()),
                Box::new(problems::Problem0045::new()),
                Box::new(problems::Problem0046::new()),
                Box::new(problems::Problem0047::new()),
                Box::new(problems::Problem0048::new()),
                Box::new(problems::Problem0049::new()),
                Box::new(problems::Problem0050::new()),
                Box::new(problems::Problem0052::new()),
                Box::new(problems::Problem0053::new()),
                Box::new(problems::Problem0054::new()),
                Box::new(problems::Problem0055::new()),
                Box::new(problems::Problem0056::new()),
                Box::new(problems::Problem0057::new()),
                Box::new(problems::Problem0058::new()),
                Box::new(problems::Problem0059::new()),
                Box::new(problems::Problem0061::new()),
                Box::new(problems::Problem0062::new()),
                Box::new(problems::Problem0063::new()),
                Box::new(problems::Problem0064::new()),
                Box::new(problems::Problem0065::new()),
                Box::new(problems::Problem0066::new()),
                Box::new(problems::Problem0067::new()),
                Box::new(problems::Problem0069::new()),
                Box::new(problems::Problem0070::new()),
                Box::new(problems::Problem0071::new()),
                Box::new(problems::Problem0072::new()),
                Box::new(problems::Problem0073::new()),
                Box::new(problems::Problem0074::new()),
                Box::new(problems::Problem0075::new()),
                Box::new(problems::Problem0076::new()),
                Box::new(problems::Problem0077::new()),
                Box::new(problems::Problem0078::new()),
                Box::new(problems::Problem0080::new()),
                Box::new(problems::Problem0081::new()),
                Box::new(problems::Problem0082::new()),
                Box::new(problems::Problem0085::new()),
                Box::new(problems::Problem0089::new()),
                Box::new(problems::Problem0091::new()),
                Box::new(problems::Problem0092::new()),
                Box::new(problems::Problem0095::new()),
                Box::new(problems::Problem0097::new()),
                Box::new(problems::Problem0098::new()),
                Box::new(problems::Problem0099::new()),
                Box::new(problems::Problem0100::new()),
                Box::new(problems::Problem0102::new()),
            ],
        };
        new_obj
            .problems
            .sort_unstable_by_key(|problem| problem.id());
        new_obj
    }
}
impl Default for PEuler {
    fn default() -> Self {
        Self::new()
    }
}
impl ProjectEuler for PEuler {
    fn problems<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Problem> + 'a> {
        Box::new(self.problems.iter().map(|problem| problem.as_ref()))
    }
}
