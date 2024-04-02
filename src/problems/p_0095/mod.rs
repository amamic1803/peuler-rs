//! **Problem 95** - *Amicable Chains*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(95, "Amicable Chains", solve)
}

use crate::shared::math::sum_of_proper_divisors_1_to_n;
const LIMIT: usize = 1_000_000; // upper limit for numbers to check

fn solve() -> String {
    // find the sum of proper divisors for each number 1 to LIMIT
    let divisors_sums = sum_of_proper_divisors_1_to_n(LIMIT as u64);

    // vector to store the length of the amicable chain for each number
    let mut chain_lengths = vec![None; LIMIT + 1];

    // set the chain length for 0 and 1 to 0
    chain_lengths[0] = Some(0);
    chain_lengths[1] = Some(0);

    // vector for tracking and detecting cycles
    let mut stack = Vec::new();

    // process all numbers 2 to LIMIT
    for i in 2..=LIMIT {
        // if the chain length for this number is not yet known, process it
        if chain_lengths[i].is_none() {
            // set the current number to i and process it
            let mut curr_num = i;

            // stack doesn't need to be cleared, as it will be empty at the start of each iteration

            // loop until the current number is greater than LIMIT
            // (in which case all numbers in the stack will be set to chain length 0)
            // and chain length for the current number is not yet known
            // (if it is, it means that all numbers prior to it in the stack aren't in a cycle)
            // (also set their chain length to the zero)
            while curr_num <= LIMIT && chain_lengths[curr_num].is_none() {
                // check if the current number is already in the stack
                // if it isn't add it to the stack
                // if it is, calculate the chain length for all numbers in the stack
                match stack.iter().position(|&x| x == curr_num) {
                    Some(pos) => {
                        // determine the length of the chain
                        let chain_len = stack.len() - pos;
                        // set the chain length for all numbers in the stack that are in the cycle
                        for stack_elem in stack.drain(pos..) {
                            chain_lengths[stack_elem] = Some(chain_len);
                        }
                        // other values in the stack are not in the cycle, so they will be set to 0 after the loop
                        break;
                    }
                    None => stack.push(curr_num),
                }
                // set the current number to the sum of its proper divisors (next number in the possible chain)
                curr_num = divisors_sums[curr_num] as usize;
            }
            // if any elements in the stack are left, set their chain length to 0
            for stack_num in stack.drain(..) {
                chain_lengths[stack_num] = Some(0);
            }
        }
    }

    // find the longest chain (maximum value in chain_lengths)
    let max_chain_length = *chain_lengths.iter().flatten().max().unwrap();
    // find the first element with that chain length and return it
    for (num, length) in chain_lengths
        .into_iter()
        .enumerate()
        .filter_map(|(i, length)| length.map(|length| (i, length)))
    {
        if length == max_chain_length {
            return num.to_string();
        }
    }
    unreachable!("Maximum value was determined from chain lengths, therefore at least one chain of that length must exist.")
}
