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
pub mod p_0013;
pub mod p_0014;
pub mod p_0015;
pub mod p_0017;
pub mod p_0018;
pub mod p_0024;
pub mod p_0025;
pub mod p_0028;
pub mod p_0033;
pub mod p_0048;
pub mod p_0054;
pub mod p_0056;
pub mod p_0057;
pub mod p_0058;
pub mod p_0065;
pub mod p_0067;
pub mod p_0081;

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
use p_0013::get_problem as problem_0013;
use p_0014::get_problem as problem_0014;
use p_0015::get_problem as problem_0015;
use p_0017::get_problem as problem_0017;
use p_0018::get_problem as problem_0018;
use p_0024::get_problem as problem_0024;
use p_0025::get_problem as problem_0025;
use p_0028::get_problem as problem_0028;
use p_0033::get_problem as problem_0033;
use p_0048::get_problem as problem_0048;
use p_0054::get_problem as problem_0054;
use p_0056::get_problem as problem_0056;
use p_0057::get_problem as problem_0057;
use p_0058::get_problem as problem_0058;
use p_0065::get_problem as problem_0065;
use p_0067::get_problem as problem_0067;
use p_0081::get_problem as problem_0081;


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
        problem_0013(),
        problem_0014(),
        problem_0015(),
        problem_0017(),
        problem_0018(),
        problem_0024(),
        problem_0025(),
        problem_0028(),
        problem_0033(),
        problem_0048(),
        problem_0054(),
        problem_0056(),
        problem_0057(),
        problem_0058(),
        problem_0065(),
        problem_0067(),
        problem_0081(),
    ])
}