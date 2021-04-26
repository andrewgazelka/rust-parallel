use std::cmp::{max, min};

// use itertools::Itertools;
// use rand::Rng;

// use crate::par_poly::calculate;
// use crate::pref_sum_mod::pref_sum;

mod k_means;
mod points;
mod pref_sum_mod;
mod par_poly;
mod sort;

fn main() {

    // let pow = 3;
    // let n = 2 << pow;
    // let mut rng = rand::thread_rng();
    //
    // let rand_vec: Vec<i32> = (0..n).map(|_| rng.gen_range(0..32)).collect();
    //
    // let mut vecs = vec![rand_vec];
    //
    // for _ in 0..(pow + 1) {
    //     let mut vecs_new = Vec::new();
    //     for vec in vecs {
    //         let (a, b) = bitonic(&vec);
    //         vecs_new.push(a);
    //         vecs_new.push(b);
    //     }
    //     vecs = vecs_new;
    // }


    // let res = vecs.iter().flatten().collect_vec();
    // println!("{:?}", vecs);
}
