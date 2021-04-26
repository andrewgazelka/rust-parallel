//! Odd even merge-sort implementation implemented in [odd_even_mergesort]

use rayon::prelude::{ParallelIterator, ParallelSliceMut};

use crate::sort::utils::is_pow_2;
use crate::sort::Sorter;

pub struct OddEvenMergeSort;

impl Sorter for OddEvenMergeSort {
    type T = i32;
    fn sort(&self, arr: &mut [Self::T]) {
        odd_even_mergesort(arr);
    }
}

/// Apply a parallel odd-even mergesort.
fn odd_even_mergesort(array: &mut [i32]) {

    // we assert that we have perfect splits (array is power of 2). This algorithm requires perfect splits.
    assert!(is_pow_2(array.len()));

    sort_pairs_of_2(array);


    // the current size of the arrays we will merge together. If this is greater than half the length of the array
    // we will not be able to merge
    let mut merge_size = 2;

    let half_len = array.len();
    while merge_size < half_len {

        // we need to grab chunks twice the size of the arrays we will merge
        let chunk_size = merge_size * 2;

        array.par_chunks_mut(chunk_size).for_each(|input| {
            // we split the input in two to get the two arrays we need to merge
            let (first_half, second_half) = split(input);
            let res = o_emerge(first_half, second_half);

            // this sets all the values of `input` to be that of res
            input.clone_from_slice(&res);
        });

        // we consecutively merge larger and larger arrays
        merge_size *= 2;
    }
}


/// returns 0th, 2nd, 4th, ... elements
/// - Equivalent to `E(A)`
fn left_sample(arr: &[i32]) -> Vec<i32> {
    arr.iter().cloned().step_by(2).collect()
}

/// return 1st, 3rd, 5th, ... elements
/// - Equivalent to `O(A)`
fn right_sample(arr: &[i32]) -> Vec<i32> {
    arr.iter().cloned().skip(1).step_by(2).collect()
}

/// # Input
/// Two sorted arrays of the same length—n.
/// - a
/// - b
/// # Output
/// Sorted array which is the union of a,b
fn o_emerge(a: &[i32], b: &[i32]) -> Vec<i32> {
    assert_eq!(a.len(), b.len());
    let n = a.len();
    assert!(n > 0);

    // since we can break in half anymore we must have a special case
    if n == 1 {
        let mut first = a[0];
        let mut second = b[0];
        sort_pair(&mut first, &mut second);
        return vec![first, second];
    }

    let c = o_emerge(&left_sample(a), &right_sample(b));
    let d = o_emerge(&right_sample(a), &left_sample(b));

    let mut sorted = Vec::with_capacity(2 * n);
    for i in 0..n {
        let mut first = c[i];
        let mut second = d[i];
        sort_pair(&mut first, &mut second);
        sorted.push(first);
        sorted.push(second);
    }

    sorted
}

fn sort_pair(a: &mut i32, b: &mut i32) {
    if a > b {
        std::mem::swap(a, b);
    }
}

/// sort pairs of 2. This is done in parallel (one thread per chunk of 2). It *should* be an O(1) operation
fn sort_pairs_of_2(array: &mut [i32]) {
    array.par_chunks_mut(2).for_each(|input| {
        if let [a, b] = input {
            sort_pair(a, b);
        } else { panic!("chunk is not of size 2") }
    });
}

fn split(input: &[i32]) -> (&[i32], &[i32]) {
    let len = input.len();
    let half_len = len / 2;
    assert_eq!(half_len * 2, len); // perfect split
    (&input[..half_len], &input[half_len..])
}

#[cfg(test)]
mod tests {
    use crate::sort::even_odd::{left_sample, o_emerge, odd_even_mergesort, right_sample};
    use crate::sort::utils::is_sorted;

    #[test]
    fn left_sample_works() {
        let input = vec![1, 2, 5, 43];
        assert_eq!(left_sample(&input), vec![1, 5]);

        let input = vec![1, 5];
        assert_eq!(left_sample(&input), vec![1]);
    }

    #[test]
    fn right_sample_works() {
        let input = vec![1, 2, 5, 43];
        assert_eq!(right_sample(&input), vec![2, 43]);

        let input = vec![2, 43];
        assert_eq!(right_sample(&input), vec![43]);
    }

    #[test]
    fn merge_works() {
        let input = vec![1, 1, 5, 43];
        let half_way = 2;
        let first_half = &input[0..half_way];
        let second_half = &input[half_way..];
        assert_eq!(first_half.len(), 2);
        assert_eq!(second_half.len(), 2);

        let res = o_emerge(first_half, second_half);
        assert!(is_sorted(&res));
    }

    #[test]
    fn merge_sort_works() {
        let mut arr = [1, 1, 43, 5, 6, 62, 7, 7];
        odd_even_mergesort(&mut arr);
        assert!(is_sorted(&arr));
    }
}
