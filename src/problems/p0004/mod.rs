use crate::Problem;
use crate::math::digits::is_palindrome;

problem!(Problem0004, 4, "Largest Palindrome Product");
impl Problem for Problem0004 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let mut largest_palindrome: u64 = 0;

        for fact1 in 100..1000 {
            for fact2 in fact1..1000 {
                let product = fact1 * fact2;
                if is_palindrome(product, 10) && (product > largest_palindrome) {
                    largest_palindrome = product;
                }
            }
        }

        largest_palindrome.to_string()
    }
}
