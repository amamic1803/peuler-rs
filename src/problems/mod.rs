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
pub mod p_0012;
pub mod p_0013;
pub mod p_0014;
pub mod p_0015;
pub mod p_0016;
pub mod p_0017;
pub mod p_0018;
pub mod p_0019;
pub mod p_0020;
pub mod p_0021;
pub mod p_0022;
pub mod p_0023;
pub mod p_0024;
pub mod p_0025;
pub mod p_0027;
pub mod p_0028;
pub mod p_0029;
pub mod p_0030;
pub mod p_0031;
pub mod p_0033;
pub mod p_0034;
pub mod p_0035;
pub mod p_0036;
pub mod p_0042;
pub mod p_0045;
pub mod p_0048;
pub mod p_0050;
pub mod p_0053;
pub mod p_0054;
pub mod p_0055;
pub mod p_0056;
pub mod p_0057;
pub mod p_0058;
pub mod p_0059;
pub mod p_0063;
pub mod p_0065;
pub mod p_0067;
pub mod p_0081;
pub mod p_0091;
pub mod p_0092;
pub mod p_0097;
pub mod p_0099;
pub mod p_0102;

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
use p_0012::get_problem as problem_0012;
use p_0013::get_problem as problem_0013;
use p_0014::get_problem as problem_0014;
use p_0015::get_problem as problem_0015;
use p_0016::get_problem as problem_0016;
use p_0017::get_problem as problem_0017;
use p_0018::get_problem as problem_0018;
use p_0019::get_problem as problem_0019;
use p_0020::get_problem as problem_0020;
use p_0021::get_problem as problem_0021;
use p_0022::get_problem as problem_0022;
use p_0023::get_problem as problem_0023;
use p_0024::get_problem as problem_0024;
use p_0025::get_problem as problem_0025;
use p_0027::get_problem as problem_0027;
use p_0028::get_problem as problem_0028;
use p_0029::get_problem as problem_0029;
use p_0030::get_problem as problem_0030;
use p_0031::get_problem as problem_0031;
use p_0033::get_problem as problem_0033;
use p_0034::get_problem as problem_0034;
use p_0035::get_problem as problem_0035;
use p_0036::get_problem as problem_0036;
use p_0042::get_problem as problem_0042;
use p_0045::get_problem as problem_0045;
use p_0048::get_problem as problem_0048;
use p_0050::get_problem as problem_0050;
use p_0053::get_problem as problem_0053;
use p_0054::get_problem as problem_0054;
use p_0055::get_problem as problem_0055;
use p_0056::get_problem as problem_0056;
use p_0057::get_problem as problem_0057;
use p_0058::get_problem as problem_0058;
use p_0059::get_problem as problem_0059;
use p_0063::get_problem as problem_0063;
use p_0065::get_problem as problem_0065;
use p_0067::get_problem as problem_0067;
use p_0081::get_problem as problem_0081;
use p_0091::get_problem as problem_0091;
use p_0092::get_problem as problem_0092;
use p_0097::get_problem as problem_0097;
use p_0099::get_problem as problem_0099;
use p_0102::get_problem as problem_0102;


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
        problem_0012(),
        problem_0013(),
        problem_0014(),
        problem_0015(),
        problem_0016(),
        problem_0017(),
        problem_0018(),
        problem_0019(),
        problem_0020(),
        problem_0021(),
        problem_0022(),
        problem_0023(),
        problem_0024(),
        problem_0025(),
        problem_0027(),
        problem_0028(),
        problem_0029(),
        problem_0030(),
        problem_0031(),
        problem_0033(),
        problem_0034(),
        problem_0035(),
        problem_0036(),
        problem_0042(),
        problem_0045(),
        problem_0048(),
        problem_0050(),
        problem_0053(),
        problem_0054(),
        problem_0055(),
        problem_0056(),
        problem_0057(),
        problem_0058(),
        problem_0059(),
        problem_0063(),
        problem_0065(),
        problem_0067(),
        problem_0081(),
        problem_0091(),
        problem_0092(),
        problem_0097(),
        problem_0099(),
        problem_0102(),
    ])
}
