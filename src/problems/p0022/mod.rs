use crate::Problem;

problem!(Problem0022, 22, "Names Scores");

impl Problem for Problem0022 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let input = include_str!("0022_names.txt");
        let names = parse_input(input);

        let mut result = 0;

        for (i, name) in names.into_iter().enumerate() {
            result += (i + 1) as u32 * name_value(name);
        }

        result.to_string()
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    let mut input: Vec<&str> = input
        .trim()
        .split(',')
        .map(|s| s.trim_matches('"'))
        .collect();

    input.sort_unstable();

    input
}

fn name_value(name: &str) -> u32 {
    name.chars().map(letter_value).sum()
}

fn letter_value(letter: char) -> u32 {
    letter as u32 - 'A' as u32 + 1
}
