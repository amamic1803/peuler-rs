use peuler::{PEuler, ProjectEuler};

pub fn run_test(problem_id: usize, expected: &str) {
    //! Run test for a problem.

    let problems = PEuler::new();
    let result = problems.run(problem_id).unwrap();

    assert_eq!(result.trim(), expected.trim());
}
