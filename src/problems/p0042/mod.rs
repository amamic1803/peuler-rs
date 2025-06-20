use crate::Problem;

problem!(Problem0042, 42, "Coded Triangle Numbers");

impl Problem for Problem0042 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let input = include_str!("0042_words.txt");

        input
            .trim()
            .split(',')
            .map(|word| {
                word.trim_matches('"')
                    .chars()
                    .map(letter_value)
                    .sum::<u32>()
            })
            .filter(is_triangle_num)
            .count()
            .to_string()
    }
}

fn letter_value(letter: char) -> u32 {
    letter as u32 - 'A' as u32 + 1
}

fn is_triangle_num(n: &u32) -> bool {
    // T(n) = n(n+1)/2
    // 2T(n) = n(n+1)
    // 2T(n) = n^2 + n
    // 0 = n^2 + n - 2T(n)
    // n = (-1 + sqrt(1 + 8T(n))) / 2
    // if we get n to be an integer, then T(n) is a triangle number

    let n = (-1.0 + ((1 + 8 * n) as f64).sqrt()) / 2.0;

    (n.round() - n).abs() < 1e-10
}
