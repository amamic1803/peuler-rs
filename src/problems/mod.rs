//! This module contains all the problems.

use crate::shared::structures::Problems;


pub mod p_0001;
pub mod p_0002;
pub mod p_0003;
pub mod p_0004;
pub mod p_0005;

use p_0001::get_problem as problem_0001;
use p_0002::get_problem as problem_0002;
use p_0003::get_problem as problem_0003;
use p_0004::get_problem as problem_0004;
use p_0005::get_problem as problem_0005;


/// Returns a `Problems` struct containing all the problems.
pub fn get_problems() -> Problems {
    Problems::new(vec![
        problem_0001(),
        problem_0002(),
        problem_0003(),
        problem_0004(),
        problem_0005(),
    ])
}