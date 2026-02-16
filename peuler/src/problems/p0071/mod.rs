use crate::Solution;

problem!(Problem0071, 71, "Ordered Fractions");

impl Solution for Problem0071 {
    fn solve(&self) -> String {
        const MAX: u32 = 1_000_000;
        const C: u32 = 3;
        const D: u32 = 7;

        // the sequence of these fractions is called Farey sequence
        // the order of Farey sequence is the maximum value of the denominator
        // the neighbours a/b and c/d in the Farey (order max(b,d)) sequence satisfy bc - ad = 1
        // the next fraction that would appear between a/b and c/d is (a+c)/(b+d)
        // so we just need to find the neighbour that is smaller than 3/7
        // and check that b + d > 1_000_000 (that means that there is no fraction between them in the Farey sequence)
        // plugging in the C and D values we get:
        // 3b - 7a = 1
        // a = (3b - 1) / 7
        // so we want b to be the largest possible value, and also 3b - 1 to be divisible by 7

        let mut b = MAX;
        while !(C * b - 1).is_multiple_of(D) {
            b -= 1;
        }

        // then we just need to calculate a (the numerator of the fraction)
        let a = (C * b - 1) / D;

        // return the numerator of the fraction
        a.to_string()
    }
}
