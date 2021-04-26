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
