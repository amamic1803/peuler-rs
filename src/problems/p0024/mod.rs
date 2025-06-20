use crate::Problem;
use crate::math::factorial;

problem!(Problem0024, 24, "Lexicographic Permutations");

impl Problem for Problem0024 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let mut digits: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut permutations = 999_999; // 1_000_000 - 1 because the first permutation is the original
        let mut result = String::new();

        while permutations != 0 {
            // factorial of the number of digits minus 1
            // this is the number of permutations for digits after the first digit
            let fact = factorial(digits.len() - 1);

            // we divide the number of permutations by the factorial
            // this is because if there are more permutations left than the factorial
            // we need to take the next digit as the first one
            // and since the digits are sorted, we can just take the index
            let index = permutations / fact;

            // add the digit to the result (since we are now taking it as the first digit)
            result.push_str(&digits[index].to_string());

            // remove that digit from the list of digits
            digits.remove(index);

            // because the initial vector is sorted, we don't need to sort digits again
            // as we aren't actually permutating the vector of digits

            // reduce the number of permutations left
            // note that if the index is zero, we aren't reducing the number of permutations
            // that is because we are taking the first digit as the first digit
            // so we aren't actually moving to a new permutation
            permutations -= index * fact;
        }

        // add the remaining digits to the result
        for i in digits {
            result.push_str(&i.to_string());
        }

        result.to_string()
    }
}
