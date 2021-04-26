use rayon::prelude::ParallelIterator;
use rayon::slice::ParallelSliceMut;

pub fn is_sorted(elems: &[i32]) -> bool {
    let mut previous = i32::MIN;

    for &elem in elems {
        if elem < previous {
            return false;
        }
        previous = elem;
    }
    return true;
}

pub fn is_pow_2(elem: usize) -> bool {
    let mut on = elem;
    if on > 0 {
        on = on & (on - 1)
    }
    on == 0
}


#[cfg(test)]
mod tests {
    use crate::sort::utils::is_sorted;

    #[test]
    fn it_works() {
        assert!(is_sorted(&[1, 5, 8, 8, 9]));
        assert!(!is_sorted(&[1, 5, 8, 7, 9]));
    }
}

pub fn split(input: &[i32]) -> (&[i32], &[i32]) {
    let len = input.len();
    let half_len = len / 2;
    assert_eq!(half_len * 2, len); // perfect split
    input.split_at(half_len)
}

pub fn split_mut(input: &mut [i32]) -> (&mut [i32], &mut [i32]) {
    let len = input.len();
    let half_len = len / 2;
    assert_eq!(half_len * 2, len); // perfect split
    input.split_at_mut(half_len)
}


#[derive(Copy, Clone)]
pub enum SortType {
    Greater = 0,
    Less,
}

impl SortType {
    fn neg(&self) -> SortType {
        match self {
            SortType::Greater => SortType::Less,
            SortType::Less => SortType::Greater,
        }
    }
}

pub fn sort_pair(a: &mut i32, sort_type: SortType, b: &mut i32) {
    let swap = match sort_type {
        SortType::Greater => a > b,
        SortType::Less => a < b
    };

    if swap {
        std::mem::swap(a, b);
    }
}

/// sort pairs of 2. This is done in parallel (one thread per chunk of 2). It *should* be an O(1) operation
pub fn sort_pairs_of_2(array: &mut [i32], sort_type: SortType) {
    array.par_chunks_mut(2).for_each(|input| {
        if let [a, b] = input {
            sort_pair(a, sort_type, b);
        } else { panic!("chunk is not of size 2") }
    });
}
