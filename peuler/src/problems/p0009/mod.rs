use crate::Solution;

problem!(Problem0009, 9, "Special Pythagorean Triplet");

impl Solution for Problem0009 {
    fn solve(&self) -> String {
        const LIMIT: i32 = 1000;

        for a in 1..(LIMIT / 3 + 1) {
            for b in a..((LIMIT - a) / 2 + 1) {
                let c = LIMIT - a - b;
                if a * a + b * b == c * c {
                    return (a * b * c).to_string();
                }
            }
        }

        panic!("No solution found");
    }
}
