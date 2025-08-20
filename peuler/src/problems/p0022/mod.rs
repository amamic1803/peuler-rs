use crate::Solution;

problem!(Problem0022, 22, "Names Scores");

impl Solution for Problem0022 {
    fn solve(&self) -> String {
        const INPUT: &str = include_str!("0022_names.txt");
        let mut names = INPUT
            .trim()
            .split(',')
            .map(|s| s.trim_matches('"'))
            .collect::<Vec<_>>();
        names.sort_unstable();

        names
            .into_iter()
            .enumerate()
            .map(|(i, name)| {
                (i + 1) as u32 * name.chars().map(|c| c as u32 - 'A' as u32 + 1).sum::<u32>()
            })
            .sum::<u32>()
            .to_string()
    }
}
