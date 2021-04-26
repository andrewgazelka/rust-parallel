use rayon::iter::ParallelIterator;
use rayon::prelude::{IndexedParallelIterator, ParallelSliceMut};

use crate::sort::Sorter;
use crate::sort::utils::{is_pow_2, sort_pair, SortType, split, split_mut};
use crate::sort::utils::SortType::{Greater, Less};

struct BitonicSorter;

impl Sorter for BitonicSorter {
    type T = i32;

    fn sort(&self, arr: &mut [Self::T]) {
        assert!(is_pow_2(arr.len()));

        // phase 1
        bitonic_4_seq(arr);

        // phase 2
        let mut stride = arr.len() / 4;
        while stride >= 1 {
            bitonic_merge(arr, stride);
            stride /= 2;
        }

        let mut stride = arr.len() / 2;
        while stride >= 1 {
            cmp_exchange(arr, stride);
            stride /= 2;
        }
    }
}

fn bitonic_4_seq(arr: &mut [i32]) {
    arr.par_chunks_mut(4).for_each(|slice| {
        if let [a, b, c, d] = slice {
            bitonic_4(a, b, c, d);
        } else {
            panic!("chunk is not a group of 4")
        }
    })
}


fn bitonic_4(a: &mut i32, b: &mut i32, c: &mut i32, d: &mut i32) {
    sort_pair(a, Greater, b);
    sort_pair(c, Less, d);
}

fn merge_helper(arr: &mut [i32], order: SortType) {
    let n = arr.len() / 2;
    let (first_half, second_half) = split_mut(arr);
    for i in 0..n {
        let a = &mut first_half[i];
        let b = &mut second_half[i];
        sort_pair(a, order, b);
    }
}

fn cmp_exchange(arr: &mut [i32], stride: usize) {
    let chunk_size = stride * 2;

    arr.par_chunks_mut(chunk_size).for_each(|chunk| {
        merge_helper(chunk, Greater);
    });
}

fn bitonic_merge(arr: &mut [i32], stride: usize) {
    let chunk_size = stride * 2;

    let half_idx = arr.len() / (2 * chunk_size);

    arr.par_chunks_mut(chunk_size).enumerate().for_each(|(i, chunk)| {
        let order = if i < half_idx { Greater } else { Less };
        merge_helper(chunk, order);
    });
}

#[cfg(test)]
mod tests {
    use rand::random;

    use crate::sort::bitonic::{bitonic_4_seq, bitonic_merge, BitonicSorter, cmp_exchange};
    use crate::sort::Sorter;
    use crate::sort::utils::is_sorted;

    #[test]
    fn lecture_example_works() {
        let unsorted = &mut vec![7, 6, 2, 4, 5, 3, 8, 1];

        bitonic_4_seq(unsorted);
        assert_eq!(unsorted, &vec![6, 7, 4, 2, 3, 5, 8, 1]);

        // merging
        bitonic_merge(unsorted, 2);
        assert_eq!(unsorted, &vec![4, 2, 6, 7, 8, 5, 3, 1]);
        bitonic_merge(unsorted, 1);
        assert_eq!(unsorted, &vec![2, 4, 6, 7, 8, 5, 3, 1]);

        // exchanging
        cmp_exchange(unsorted, 4);
        assert_eq!(unsorted, &vec![2, 4, 3, 1, 8, 5, 6, 7]);
        cmp_exchange(unsorted, 2);
        assert_eq!(unsorted, &vec![2, 1, 3, 4, 6, 5, 8, 7]);

        cmp_exchange(unsorted, 1);
        assert_eq!(unsorted, &vec![1, 2, 3, 4, 5, 6, 7, 8]) // is now sorted
    }

    #[test]
    fn lecture_one_shot() {
        let arr = &mut vec![7, 6, 2, 4, 5, 3, 8, 1];
        BitonicSorter.sort(arr);
        assert_eq!(arr, &vec![1, 2, 3, 4, 5, 6, 7, 8])
    }


    #[test]
    fn random_vec() {
        let mut arr: Vec<i32> = (0..16).map(|_| random()).collect();
        BitonicSorter.sort(&mut arr);
        println!("{:?}", arr);
        assert!(is_sorted(&arr));

    }
}
