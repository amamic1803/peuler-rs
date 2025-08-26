use clap::{Arg, ArgAction, command, value_parser};
use std::io::{Write, stdout};
use std::process::ExitCode;

use peuler::{PEuler, ProjectEuler};
use pmath::statistics::Sample;

fn main() -> ExitCode {
    let argv = command!()
        .arg(
            Arg::new("problem")
                .value_name("PROBLEM")
                .help("The problem identifier")
                .required_unless_present_any(["list", "count", "solutions", "benchmark"])
                .conflicts_with_all(["list", "count", "solutions"])
                .value_parser(value_parser!(u16).range(1..)),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .action(ArgAction::SetTrue)
                .help("List all available problems")
                .required(false)
                .conflicts_with_all(["problem", "count", "solutions", "benchmark"]),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .help("Get the number of available problems")
                .required(false)
                .conflicts_with_all(["problem", "list", "solutions", "benchmark"]),
        )
        .arg(
            Arg::new("solutions")
                .short('s')
                .long("solutions")
                .action(ArgAction::SetTrue)
                .help("Calculate solutions for all available problems")
                .required(false)
                .conflicts_with_all(["problem", "list", "count", "benchmark"]),
        )
        .arg(
            Arg::new("benchmark")
                .short('b')
                .long("benchmark")
                .value_name("ITERATIONS")
                .help("Benchmark the solution for the specified problem or all problems")
                .required(false)
                .default_missing_value("100")
                .value_parser(value_parser!(u64).range(3..))
                .conflicts_with_all(["list", "count", "solutions"]),
        )
        .get_matches();

    let list_flag: bool = argv.get_flag("list");
    let count_flag: bool = argv.get_flag("count");
    let solutions_flag: bool = argv.get_flag("solutions");
    let problem_id = argv.get_one::<u16>("problem").map(|&u| u as usize);
    let benchmark_iterations = argv.get_one::<u64>("benchmark").copied();

    let project_euler = PEuler::new();

    if list_flag {
        for problem in project_euler.problems() {
            println!("Problem {:04}: {}", problem.id(), problem.title());
        }
    } else if count_flag {
        println!("{}", project_euler.problems().count());
    } else if solutions_flag {
        let max_line_len = project_euler
            .problems()
            .map(|problem| problem.title().chars().count() + 14)
            .max()
            .unwrap_or(0);
        for problem in project_euler.problems() {
            print!(
                "Problem {:04}: {:width$} => ",
                problem.id(),
                problem.title(),
                width = max_line_len - 14
            );
            let _ = stdout().flush();
            println!("{}", problem.solve());
        }
    } else if let Some(iters) = benchmark_iterations {
        match problem_id {
            Some(id) => {
                let problem = match project_euler.problem(id) {
                    Ok(problem) => problem,
                    Err(e) => {
                        eprintln!("Error: {e}");
                        return ExitCode::FAILURE;
                    }
                };

                let mut solution = String::new();
                let mut sample = Sample::new();
                for _ in 0..iters {
                    let (result, elapsed) = problem.benchmark();
                    solution = result;
                    sample.push(elapsed.as_nanos());
                }
                let mut mean = sample.mean().unwrap();
                let mut stddev = sample.sample_stddev().unwrap();
                let mut unit = "ns";
                if mean > 1000.0 {
                    mean /= 1000.0;
                    stddev /= 1000.0;
                    unit = "µs";
                }
                if mean > 1000.0 {
                    mean /= 1000.0;
                    stddev /= 1000.0;
                    unit = "ms";
                }
                if mean > 1000.0 {
                    mean /= 1000.0;
                    stddev /= 1000.0;
                    unit = "s";
                }
                println!(
                    "{solution:20} (iterations: {iters}, mean: {mean:>11.6} {unit:>2}, stddev: {stddev:>11.6} {unit:>2})"
                );
            }
            None => {
                let max_line_len = project_euler
                    .problems()
                    .map(|problem| problem.title().chars().count() + 14)
                    .max()
                    .unwrap_or(0);
                for problem in project_euler.problems() {
                    print!(
                        "Problem {:04}: {:width$} => ",
                        problem.id(),
                        problem.title(),
                        width = max_line_len - 14
                    );
                    let _ = stdout().flush();

                    let mut solution = String::new();
                    let mut sample = Sample::new();
                    for _ in 0..iters {
                        let (result, elapsed) = problem.benchmark();
                        solution = result;
                        sample.push(elapsed.as_nanos());
                    }
                    let mut mean = sample.mean().unwrap();
                    let mut stddev = sample.sample_stddev().unwrap();
                    let mut unit = "ns";
                    if mean > 1000.0 {
                        mean /= 1000.0;
                        stddev /= 1000.0;
                        unit = "µs";
                    }
                    if mean > 1000.0 {
                        mean /= 1000.0;
                        stddev /= 1000.0;
                        unit = "ms";
                    }
                    if mean > 1000.0 {
                        mean /= 1000.0;
                        stddev /= 1000.0;
                        unit = "s";
                    }
                    println!(
                        "{solution:20} (iterations: {iters}, mean: {mean:>11.6} {unit:>2}, stddev: {stddev:>11.6} {unit:>2})"
                    );
                }
            }
        }
    } else {
        match project_euler.solve(problem_id.unwrap()) {
            Ok(solution) => println!("{solution}"),
            Err(e) => {
                eprintln!("Error: {e}");
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
