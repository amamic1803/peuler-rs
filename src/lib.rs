//! A collection of solutions to the problems from [Project Euler](https://projecteuler.net/).

use std::cmp::Ordering;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::time::{Duration, Instant};

macro_rules! problem {
    ($struct_name:ident, $id:literal, $title:literal) => {
        #[doc = concat!("*", $title, "*")]
        #[derive(Copy, Clone)]
        pub struct $struct_name {
            id: usize,
            title: &'static str,
        }
        impl $struct_name {
            #[doc = concat!("Create a new [", stringify!($struct_name), "] instance.")]
            pub fn new() -> Self {
                Self {
                    id: $id,
                    title: $title,
                }
            }
        }
        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

pub mod math;
pub mod problems;

/// An enum representing the errors that can occur when using this library.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
    /// The requested problem is not available.
    UnavailableProblem,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnavailableProblem => write!(f, "The requested problem is not available."),
        }
    }
}
impl StdError for Error {}

/// A structure representing the _Project Euler_.
pub struct PEuler {
    /// A vector of problems.
    problems: Vec<Box<dyn Problem>>,
}
impl PEuler {
    /// Creates a new `PEuler` instance.
    /// # Returns
    /// A new `PEuler` instance with all problems initialized.
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
        new_obj.problems.sort_by_key(|problem| problem.id());
        new_obj
    }
}
impl Default for PEuler {
    fn default() -> Self {
        Self::new()
    }
}
impl Display for PEuler {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut max_line_len = self
            .problems
            .iter()
            .map(|problem| problem.title().chars().count())
            .max()
            .unwrap_or(0);
        if max_line_len < 21 {
            max_line_len = 21;
        } else if max_line_len % 2 == 0 {
            max_line_len += 1;
        }

        for _ in 0..max_line_len {
            write!(f, "#")?;
        }
        writeln!(f)?;

        for _ in 0..((max_line_len - 15) / 2) {
            write!(f, "#")?;
        }
        write!(f, " Project Euler ")?;
        for _ in 0..((max_line_len - 15) / 2) {
            write!(f, "#")?;
        }
        writeln!(f)?;

        for _ in 0..max_line_len {
            write!(f, "#")?;
        }
        writeln!(f)?;

        for problem in self.problems() {
            writeln!(f, "Problem {:04}: {}", problem.id(), problem.title())?;
        }

        Ok(())
    }
}
impl ProjectEuler for PEuler {
    fn problems<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Problem> + 'a> {
        Box::new(self.problems.iter().map(|problem| problem.as_ref()))
    }
}

/// A trait representing the _Project Euler_.
pub trait ProjectEuler: Send + Sync {
    /// Returns all available problems.
    /// # Returns
    /// An iterator over the problems, sorted by their ID in ascending order.
    fn problems<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Problem> + 'a>;

    /// Returns the problem with the specified ID.
    /// # Arguments
    /// * `id` - The ID of the problem to retrieve.
    /// # Returns
    /// A `Result` containing a reference to the problem if it exists or an `Error`.
    /// # Errors
    /// * `Error::UnavailableProblem` - The problem is not available.
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

    /// Runs the specified problem.
    /// # Arguments
    /// * `id` - The ID of the problem to run.
    /// # Returns
    /// A `Result` containing a string with the answer to the problem or the `Error`.
    /// # Errors
    /// * `Error::UnavailableProblem` - The problem is not available.
    fn run(&self, id: usize) -> Result<String, Error> {
        Ok(self.problem(id)?.run())
    }

    /// Benchmarks the specified problem.
    /// # Arguments
    /// * `id` - The ID of the problem to benchmark.
    /// # Returns
    /// A `Result` containing a tuple with the answer to the problem
    /// and the elapsed time or the `Error`.
    /// # Errors
    /// * `Error::UnavailableProblem` - The problem is not available.
    fn benchmark(&self, id: usize) -> Result<(String, Duration), Error> {
        Ok(self.problem(id)?.benchmark())
    }
}

/// A trait representing a problem.
pub trait Problem: Send + Sync {
    /// The identifier of the problem.
    /// # Returns
    /// An integer greater than `0` representing the problem's ID.
    fn id(&self) -> usize;

    /// The title of the problem.
    /// # Returns
    /// The title of the problem.
    fn title(&self) -> &str;

    /// Run the problem.
    /// # Returns
    /// The answer to the problem.
    fn run(&self) -> String;

    /// Benchmark the problem.
    /// # Returns
    /// The answer to the problem and the elapsed time.
    fn benchmark(&self) -> (String, Duration) {
        let instant = Instant::now();
        let result = self.run();
        let elapsed = instant.elapsed();
        (result, elapsed)
    }
}
