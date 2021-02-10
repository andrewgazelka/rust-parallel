use itertools::Itertools;

use crate::par_poly::calculate;
use crate::pref_sum_mod::pref_sum;

mod k_means;
mod points;
mod pref_sum_mod;
mod par_poly;

fn main() {
    // // let mut from = vec![1, 2, -1, 4, -1, 6, -1, 8];
    // let mut from = (1..16).collect_vec();
    // pref_sum(&mut from, 4);
    // println!("{:?}", from);
    let coeff = &mut [4.0, 5.0, 6.0, 7.0];
    let res = calculate(coeff, 3.0);
    println!("result {}", res);
}
