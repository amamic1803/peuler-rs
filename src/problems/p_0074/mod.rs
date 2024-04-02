//! **Problem 74** - *Digit Factorial Chains*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(74, "Digit Factorial Chains", solve)
}

use crate::shared::math::{digits, factorial};

const MAX: usize = 1_000_000; // limit of the problem

fn solve() -> String {
    // vector to store lengths of chains
    let mut chains = vec![0_u8; MAX];

    // add known chains
    chains[169] = 3;
    chains[363_601] = 3;
    chains[1454] = 3;
    chains[871] = 2;
    chains[45361] = 2;
    chains[872] = 2;
    chains[45362] = 2;

    // stack to store items of the chain
    let mut stack = Vec::new(); // (number, chain_length)

    // iterate over all numbers
    for i in 1..(MAX as u64) {
        // if stored length is 0, calculate it
        if chains[i as usize] == 0 {
            stack.push((i, 0_u8)); // add first item to stack

            while !stack.is_empty() {
                // check if the chain length for the last item is known
                if stack.last().unwrap().1 != 0 {
                    if stack.len() == 1 {
                        let only_item = stack.pop().unwrap();
                        chains[only_item.0 as usize] = only_item.1; // only item can be added only in the outer loop, therefore it is < MAX
                    } else {
                        let last_item = stack.pop().unwrap();
                        // if the last item is < MAX, then store its chain length
                        if last_item.0 < MAX as u64 {
                            chains[last_item.0 as usize] = last_item.1;
                        }
                        // set the chain length to the previous item
                        stack.last_mut().unwrap().1 = 1 + last_item.1;
                    }
                } else {
                    // chain length for the last item is unknown, calculate next item
                    let next_item = digits(stack.last().unwrap().0)
                        .map(|d| factorial(d as u64))
                        .sum::<u64>();

                    // if the next item is the same as the last item, then the chain length of the last item is 1
                    // if the next item is < MAX and its chain length is known, then the chain length of the last item is 1 + the chain length of the next item
                    // otherwise add the next item to the stack (with chain length 0, unknown)
                    if next_item == stack.last().unwrap().0 {
                        stack.last_mut().unwrap().1 = 1;
                    } else if next_item < MAX as u64 && chains[next_item as usize] != 0 {
                        stack.last_mut().unwrap().1 = 1 + chains[next_item as usize];
                    } else {
                        stack.push((next_item, 0));
                    }
                }
            }
        }
    }

    // count how many chains have length 60
    chains.into_iter().filter(|&c| c == 60).count().to_string()
}
