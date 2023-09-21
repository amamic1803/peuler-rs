//! This module contains all the problems.

use crate::shared::structures::Problems;


pub mod p_0001;
pub mod p_0002;
pub mod p_0003;
pub mod p_0004;
pub mod p_0005;
pub mod p_0006;
pub mod p_0007;
pub mod p_0008;
pub mod p_0009;
pub mod p_0010;
pub mod p_0011;
pub mod p_0014;
pub mod p_0018;
pub mod p_0067;

use p_0001::get_problem as problem_0001;
use p_0002::get_problem as problem_0002;
use p_0003::get_problem as problem_0003;
use p_0004::get_problem as problem_0004;
use p_0005::get_problem as problem_0005;
use p_0006::get_problem as problem_0006;
use p_0007::get_problem as problem_0007;
use p_0008::get_problem as problem_0008;
use p_0009::get_problem as problem_0009;
use p_0010::get_problem as problem_0010;
use p_0011::get_problem as problem_0011;
use p_0014::get_problem as problem_0014;
use p_0018::get_problem as problem_0018;
use p_0067::get_problem as problem_0067;


/// Returns a `Problems` struct containing all the problems.
pub fn get_problems() -> Problems {
    Problems::new(vec![
        problem_0001(),
        problem_0002(),
        problem_0003(),
        problem_0004(),
        problem_0005(),
        problem_0006(),
        problem_0007(),
        problem_0008(),
        problem_0009(),
        problem_0010(),
        problem_0011(),
        problem_0014(),
        problem_0018(),
        problem_0067(),
    ])
}