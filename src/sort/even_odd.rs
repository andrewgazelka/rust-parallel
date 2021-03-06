//! Odd even merge-sort implementation implemented in [odd_even_mergesort]

use rayon::prelude::{ParallelIterator, ParallelSliceMut};

use crate::sort::utils::{is_pow_2, split, sort_pair, sort_pairs_of_2};
use crate::sort::Sorter;
use crate::sort::utils::SortType::Greater;

pub struct OddEvenMergeSort;

impl Sorter for OddEvenMergeSort {
    type T = i32;
    fn sort(&self, arr: &mut [Self::T], log: bool) {
        odd_even_mergesort(arr, log);
    }
}

/// Apply a parallel odd-even mergesort.
fn odd_even_mergesort(array: &mut [i32], log: bool) {

    // we assert that we have perfect splits (array is power of 2). This algorithm requires perfect splits.
    assert!(is_pow_2(array.len()));

    if log {
        println!("input {:?}", array);
        println!()
    }

    sort_pairs_of_2(array, Greater);

    if log {
        println!("sort pairs of 2 {:?}", array);
        println!()
    }


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
            let res = dbg!(o_emerge(first_half, second_half));

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
/// Two sorted arrays of the same length???n.
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
        sort_pair(&mut first, Greater, &mut second);
        return vec![first, second];
    }

    let c = dbg!(o_emerge(&left_sample(a), &right_sample(b)));
    let d = dbg!(o_emerge(&right_sample(a), &left_sample(b)));

    let mut sorted = Vec::with_capacity(2 * n);
    for i in 0..n {
        let mut first = c[i];
        let mut second = d[i];
        sort_pair(&mut first, Greater, &mut second);
        sorted.push(first);
        sorted.push(second);
    }

    sorted
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
        odd_even_mergesort(&mut arr, false);
        assert!(is_sorted(&arr));
    }


    #[test]
    fn homework() {
        let mut arr = [3, 10, 1, 11, 30, 18, 8, 20];
        odd_even_mergesort(&mut arr, true);
        assert!(is_sorted(&arr));
    }
}
