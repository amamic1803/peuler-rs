use project_euler::get_problems;


pub fn run_test(problem_id: usize, expected: &str) {
    //! Run test for a problem.

    let problems = get_problems();
    let result = problems.run(problem_id);

    assert_eq!(result, expected);
}