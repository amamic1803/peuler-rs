use clap::{Arg, ArgAction, command, value_parser};

use project_euler::get_problems;

//! A module containing the structures used in the project.

use std::sync::mpsc;
use tinypool::ThreadPool;

/// A structure containing the problems.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Problems {
    /// A vector containing the problems.
    problems: Vec<Problem>,
}

impl Problems {
    /// Creates a new `Problems`.
    /// # Arguments
    /// * `problems` - The vector containing the problems.
    /// # Returns
    /// A new `Problems`.
    pub fn new(mut problems: Vec<Problem>) -> Self {
        problems.sort_by_key(|problem| problem.id);
        Self { problems }
    }

    /// Get access to the inner vector of problems.
    pub fn problems(&self) -> &Vec<Problem> {
        &self.problems
    }

    /// Returns the list of available problems.
    /// # Returns
    /// The `String` with the list of available problems.
    pub fn list(&self) -> String {
        let mut result = self.print_header();

        for problem in &self.problems {
            result.push_str(&problem.name());
            result.push('\n');
        }

        result.trim().to_string()
    }

    /// Calculates and prints the solutions for all problems.
    /// # Returns
    /// The `String` with the solutions for all problems.
    pub fn solutions(&self) -> String {
        let mut result = self.print_header();

        match ThreadPool::new(None) {
            Ok(mut pool) => {
                let (tx, rx) = mpsc::channel();
                for problem in self.problems.iter() {
                    let tx = tx.clone();
                    let problem = *problem;
                    pool.add_to_queue(move || {
                        let solution = problem.run();
                        tx.send((problem, solution)).unwrap();
                    });
                }
                pool.join();
                let mut solutions = Vec::from_iter(rx.try_iter());
                solutions.sort();
                for (problem, solution) in solutions {
                    result.push_str(&problem.name());
                    result.push('\n');
                    result.push_str(&solution);
                    result.push('\n');
                }
            }
            Err(_) => {
                for problem in &self.problems {
                    result.push_str(&problem.name());
                    result.push('\n');
                    result.push_str(&problem.run());
                    result.push('\n');
                }
            }
        }

        result.trim().to_string()
    }

    /// Runs the problem's solution function.
    /// # Arguments
    /// * `problem_id` - The problem's ID.
    /// # Returns
    /// The `String` with the problem's solution.
    /// Or a message if the problem is not available.
    pub fn run(&self, problem_id: usize) -> String {
        match self.problems.iter().find(|problem| problem.id == problem_id) {
            Some(problem) => problem.run(),
            None => format!("Problem {:04} not available!", problem_id),
        }
    }

    /// Returns the number of available problems.
    pub fn count(&self) -> usize {
        self.problems.len()
    }

    /// Generates pretty header for console output.
    /// # Returns
    /// The `String` with the header.
    fn print_header(&self) -> String {

    }
}

/// A structure containing a problem.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Problem {
    /// The problem's ID.
    pub id: usize,
    /// The problem's title.
    pub title: &'static str,
    /// The problem's solution function.
    pub solution: fn() -> String,
}
impl Problem {
    /// Creates a new `Problem`.
    /// # Arguments
    /// * `id` - The problem's ID.
    /// * `title` - The problem's title.
    /// * `solution` - The problem's solution function.
    /// # Returns
    /// A new `Problem`.
    pub fn new(id: usize, title: &'static str, solution: fn() -> String) -> Self {
        Self { id, title, solution }
    }

    /// Returns the problem's id.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the problem's title.
    pub fn title(&self) -> &'static str {
        self.title
    }

    /// Returns the problem's name (id + title, nicely formatted).
    /// # Returns
    /// The `String` with the problem's id + title.
    pub fn name(&self) -> String {
    }

    /// Runs the problem's solution function.
    /// # Returns
    /// The `String` with the problem's solution.
    pub fn run(&self) -> String {
        (self.solution)()
    }
}
impl Ord for Problem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for Problem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


fn main() {
    let argv = command!()
        .arg(
            Arg::new("problem")
                .value_name("PROBLEM")
                .help("The problem number")
                .required_unless_present_any(["list", "solutions", "count"])
                .value_parser(value_parser!(u16).range(1..)),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .action(ArgAction::SetTrue)
                .help("List all Project Euler problems")
                .required(false)
                .conflicts_with_all(["problem", "solutions", "count"]),
        )
        .arg(
            Arg::new("solutions")
                .short('s')
                .long("solutions")
                .action(ArgAction::SetTrue)
                .help("Calculate the solutions for all Project Euler problems")
                .required(false)
                .conflicts_with_all(["problem", "list", "count"]),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .help("Print the number of available Project Euler problems")
                .required(false)
                .conflicts_with_all(["problem", "list", "solutions"]),
        )
        .get_matches();

    let list_flag: bool = argv.get_flag("list");
    let solutions_flag: bool = argv.get_flag("solutions");
    let count_flag: bool = argv.get_flag("count");
    let problem_id = if list_flag || solutions_flag || count_flag {
        0
    } else {
        *argv.get_one::<u16>("problem").unwrap()
    };

    let problems = get_problems();

    if list_flag {
        println!("{}", problems.list());
    } else if solutions_flag {
        println!("{}", problems.solutions());
    } else if count_flag {
        println!("{}", problems.count());
    } else {
        println!("{}", problems.run(usize::from(problem_id)));
    }
}
