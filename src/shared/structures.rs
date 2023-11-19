//! A module containing the structures used in the project.


/// A structure containing the problems.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Problems {
    /// A vector containing the problems.
    pub problems: Vec<Problem>,
}

impl Problems {
    /// Creates a new `Problems`.
    /// # Arguments
    /// * `problems` - The vector containing the problems.
    /// # Returns
    /// A new `Problems`.
    pub fn new(mut problems: Vec<Problem>) -> Self {
        problems.sort_by_key(|problem| problem.id);
        Self {
            problems,
        }
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

        for problem in &self.problems {
            result.push_str(&problem.name());
            result.push('\n');
            result.push_str(&problem.run());
            result.push('\n');
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
            None => format!("Problem {:04} not available!\n", problem_id),
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
        let mut max_line_len = self.problems.iter().map(|problem| problem.name().chars().count()).max().unwrap();
        let mut result = String::new();
        if max_line_len < 21 {
            max_line_len = 21;
        } else if max_line_len % 2 == 0 {
            max_line_len += 1;
        }
        for _ in 0..max_line_len {
            result.push('#');
        }
        result.push('\n');
        for _ in 0..((max_line_len - 15) / 2) {
            result.push('#');
        }
        result.push_str(" Project Euler ");
        for _ in 0..((max_line_len - 15) / 2) {
            result.push('#');
        }
        result.push('\n');
        for _ in 0..max_line_len {
            result.push('#');
        }
        result.push('\n');
        result
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
        Self {
            id,
            title,
            solution,
        }
    }

    /// Returns the problem's name.
    /// # Returns
    /// The `String` with the problem's id + title.
    pub fn name(&self) -> String {
        format!("Problem {:04}: {}", self.id, self.title)
    }

    /// Runs the problem's solution function.
    /// # Returns
    /// The `String` with the problem's solution.
    pub fn run(&self) -> String {
        (self.solution)()
    }
}