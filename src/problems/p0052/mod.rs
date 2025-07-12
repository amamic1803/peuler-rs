use crate::Problem;
use crate::math::digits::digits;

problem!(Problem0052, 52, "Permuted Multiples");

impl Problem for Problem0052 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        // temp vectors for storing digits
        let mut digits1 = Vec::new();
        let mut digits2 = Vec::new();

        // n = number of digits
        for n in 1.. {
            // start = smallest number with n digits
            let start = 10_u64.pow(n - 1);
            // end = largest number with n digits (divided by 6 because we are looking for a number that is a multiple of 6)
            let end = (10_u64.pow(n) - 1) / 6;

            // check all numbers in that range
            for num in start..=end {
                // store and sort digits of num in digits1
                digits1.clear();
                for digit in digits(num, 10) {
                    digits1.push(digit);
                }
                digits1.sort();

                // check multiples of num
                for multiple in 2..=6 {
                    // store and sort digits of multiple in digits2
                    digits2.clear();
                    for digit in digits(num * multiple, 10) {
                        digits2.push(digit);
                    }
                    digits2.sort();

                    // if digits don't match, condition is not met, so break and move to next num
                    if digits1 != digits2 {
                        break;
                    }

                    // if the previous if didn't break out and the multiple is 6, then we found the number we are looking for (because none of the previous multiples broke out)
                    if multiple == 6 {
                        return num.to_string();
                    }
                }
            }
        }

        unreachable!("Previous loop can only be exited by returning a value");
    }
}
