use rayon::prelude::*;

// pub fn calculate(coefficients: &mut [f64], input: f64) -> f64 {
//     let len = coefficients.len();
//
//     // modifies coeff idx i to be result of coeff_i*input^i
//     coefficients.par_iter_mut().enumerate().map(|(idx, val)| {
//         let set = *val * input.powi(idx as i32);
//         println!("set {}", set);
//         *val = set;
//     });
//
//     println!("coeff {:?}", coefficients);
//
//     let mut chunk_size = 2;
//     // add all of them up in binary tree form
//     while chunk_size < len {
//         coefficients.par_chunks_mut(chunk_size)
//             .for_each(|x| x[0] += x[chunk_size / 2]);
//         chunk_size *= 2;
//     }
//
//     // the result
//     return coefficients[0];
// }
