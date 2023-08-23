use clap::{Arg, ArgAction, command, value_parser};

use project_euler::get_problems;


fn main() {
    let argv = command!()
        .arg(Arg::new("problem")
            .value_name("PROBLEM")
            .help("The problem number")
            .required_unless_present("list")
            .value_parser(value_parser!(u16).range(1..))
        )
        .arg(Arg::new("list")
            .short('l')
            .long("list")
            .action(ArgAction::SetTrue)
            .help("List all Project Euler problems")
            .required(false)
        )
        .get_matches();

    let list_flag: bool = argv.get_flag("list");
    let problem_id = if list_flag { 0 } else { *argv.get_one::<u16>("problem").unwrap() };

    let problems = get_problems();

    if list_flag {
        println!("{}", problems.list());
    } else {
        println!("{}", problems.run(usize::from(problem_id)));
    }
}
