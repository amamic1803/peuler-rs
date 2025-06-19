//! **Problem 44** - *Pentagon Numbers*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(44, "Pentagon Numbers", solve)
}

use std::collections::HashSet;

fn solve() -> String {
    // P(n + 1) - P(n) = 3n + 1
    // that means that difference between current and next pentagonal number increases by 3 each new generated number
    // this can be used to generate pentagonal numbers easily, just by tracking the difference between current and
    // next number and adding it to the last number (and increasing the difference by 3)

    // initialise sequence with first two pentagonal numbers and difference with 4 (5 - 1)
    let mut sequence = vec![1, 5];
    let mut curr_diff = 4;
    // we also store same numbers in a set for faster lookup
    let mut sequence_set = HashSet::new();
    sequence_set.insert(1);
    sequence_set.insert(5);

    // now we will be checking each number in sequence
    // first we add to sequence all pentagonal numbers up to the sum of current and previous pentagonal number
    // (because we will be checking that sum)
    // then for every pentagonal number smaller than current one we check if it satisfies the property
    // (that is, if the sum and difference of those two numbers are pentagonal numbers)
    // but, we do that in reverse order, because we want to find the smallest difference and
    // smallest difference is current number minus previous number
    // when we find such number, we return it

    for curr_ind in 1.. {
        // find the sum of current and previous pentagonal number
        let sum = sequence[curr_ind] + sequence[curr_ind - 1];
        // while last pentagonal number in sequence is less than sum, generate and add new pentagonal numbers
        while sequence[sequence.len() - 1] < sum {
            curr_diff += 3;
            let next = sequence[sequence.len() - 1] + curr_diff;
            sequence.push(next);
            sequence_set.insert(next);
        }

        // check current number with all previous numbers (in reverse order)
        for i in (0..curr_ind).rev() {
            if sequence_set.contains(&(sequence[curr_ind] - sequence[i]))
                && sequence_set.contains(&(sequence[curr_ind] + sequence[i]))
            {
                return (sequence[curr_ind] - sequence[i]).to_string();
            }
        }
    }

    unreachable!("Previous loop is infinite and can only be stopped by returning a value");
}
