use crate::Problem;
use crate::math::digits::digits;
use std::collections::{HashMap, HashSet};

problem!(Problem0062, 62, "Cubic Permutations");

impl Problem for Problem0062 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        const PERM_COUNT: usize = 5;

        // the problem can be separated into classes of numbers with the same number of digits
        // and then we can find all cubes with that given number of digits
        // for example:
        // 1 digit: 1, 8
        // 2 digits: 27, 64
        // 3 digits: 125, 216, 343, 512, 729
        // 4 digits: 1000, 1331, 1728, 2197, 2744, 3375, 4096, 4913, 5832, 6859, 8000, 9261
        // we process class by class, until we find a class that contains a cube with 5 permutations
        // we count how many cubes with same digits there are in each class
        // and if we find that some collection of digits has 5 cubes with those digits
        // we find the smallest cube with those digits and return it

        // number from which the cubes are calculated (n -> n^3)
        let mut curr_num: u64 = 1;

        // cube of curr_num
        let mut curr_cube = curr_num.pow(3);

        // vector that will store all cubes with the same number of digits
        let mut cubes = Vec::new();

        // start with class of 1 digit numbers and go up
        for digits_count in 1.. {
            // make sure we start with an empty vector
            cubes.clear();
            // upper bound for the current class (for example, 10^1 for 1 digit numbers)
            let upper_bound = 10_u64.pow(digits_count);
            // generate all cubes with the same number of digits
            while curr_cube < upper_bound {
                cubes.push(curr_cube);
                curr_num += 1;
                curr_cube = curr_num.pow(3);
            }

            // hashmap that will store how many cubes contain some collection of digits
            let mut cube_counts = HashMap::new();

            // for each cube, we get its digits, sort them, and add 1 to the count of that collection of digits
            for cube in &cubes {
                let mut cube_digits = digits(*cube, 10).collect::<Vec<_>>();
                cube_digits.sort();
                cube_counts
                    .entry(cube_digits)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }

            // the class that contains the solution may contain more than one collection of 5 cubes with the same digits
            // so we collection of digits for each of those in a set
            let mut good_dig_seq = HashSet::new();
            for (digits, count) in cube_counts {
                if count == PERM_COUNT {
                    good_dig_seq.insert(digits);
                }
            }

            // now we loop over cubes, calculate their digits, sort them, and check if they are in the set of good collections of digits
            // the first cube that is in the set is the solution (because we are processing cubes in ascending order)
            for cube in &cubes {
                let mut cube_digits = digits(*cube, 10).collect::<Vec<_>>();
                cube_digits.sort();

                if good_dig_seq.contains(&cube_digits) {
                    return cube.to_string();
                }
            }
        }

        unreachable!("Previous loop is infinite and can only be exited by returning.")
    }
}
