mod utils;
pub mod even_odd;
pub mod bitonic;

trait Sorter {
    type T;
    fn sort(&self, arr: &mut [Self::T]);
}
