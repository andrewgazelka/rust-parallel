use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::prelude::{ParallelSlice, ParallelSliceMut};

/// Takes a vector and modifies it to contain sub-pref sequence sequentially
fn pref_sum_seq(vec: &mut [i32], init_on: i32) {
    let mut on = init_on;

    for x in vec {
        on += *x;
        *x = on;
    }
}

/// Takes a vector and modifies it to contain sub-pref sequence in parallel
pub fn pref_sum(vec: &mut [i32], threads: usize) {

    let len = vec.len();
    let chunk_size = len / threads;

    // computes sub-sums in n-threads
    let sub_prefs = {
        let mut sums: Vec<i32> = vec.par_chunks(chunk_size)
            .map(|slice| slice.iter().sum())
            .collect();

        pref_sum_seq(&mut sums, 0);
        sums
    };


    // in n-threads uses sub-sums to indepedently compute pref_sum_seq
    vec.par_chunks_mut(chunk_size).enumerate()
        .for_each(|(i, slice)| {
            let pref_sum_before = if i == 0 { 0 } else { sub_prefs[i - 1] };
            pref_sum_seq(slice, pref_sum_before);
        });
}
