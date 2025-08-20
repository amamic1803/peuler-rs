use crate::Solution;

problem!(Problem0042, 42, "Coded Triangle Numbers");

impl Solution for Problem0042 {
    fn solve(&self) -> String {
        let input = include_str!("0042_words.txt");

        input
            .trim()
            .split(',')
            .map(|word| {
                word.trim_matches('"')
                    .chars()
                    .map(|c| c as u32 - 'A' as u32 + 1)
                    .sum::<u32>()
            })
            .filter(is_triangle_num)
            .count()
            .to_string()
    }
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
