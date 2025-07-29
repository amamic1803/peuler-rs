use clap::{Arg, ArgAction, command, value_parser};

use peuler::PEuler;
use peuler::ProjectEuler;

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

    let problems = PEuler::new();

    if list_flag {
        println!("{problems}");
    } else if solutions_flag {
        println!("sols");
    } else if count_flag {
        println!("cnt:");
    } else {
        println!("{}", problems.run(usize::from(problem_id)).unwrap());
    }
}
