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
            problems: vec![Box::new(problems::Problem0001::new())],
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
        write!(f, "\n")?;

        for _ in 0..((max_line_len - 15) / 2) {
            write!(f, "#")?;
        }
        write!(f, " Project Euler ")?;
        for _ in 0..((max_line_len - 15) / 2) {
            write!(f, "#")?;
        }
        write!(f, "\n")?;

        for _ in 0..max_line_len {
            write!(f, "#")?;
        }
        write!(f, "\n")?;

        for problem in self.problems() {
            writeln!(f, "Problem {:04}: {}", problem.id(), problem.title())?;
        }

        Ok(())
    }
}
impl ProjectEuler for PEuler {
    fn problems<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Problem> + 'a> {
        Box::new(self.problems.iter().map(|year| year.as_ref()))
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
    /// An integer greater than 0 representing the problem's ID.
    fn id(&self) -> usize;

    /// The title of the problem.
    /// # Returns
    /// A string containing the title.
    fn title(&self) -> &str;

    /// Runs the problem.
    /// # Returns
    /// A string with the answer to the problem.
    fn run(&self) -> String;

    /// Benchmarks the problem.
    /// # Returns
    /// A tuple with the answer to the problem and the elapsed time.
    fn benchmark(&self) -> (String, Duration) {
        let instant = Instant::now();
        let result = self.run();
        let elapsed = instant.elapsed();
        (result, elapsed)
    }
}
