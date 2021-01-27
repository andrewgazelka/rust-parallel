use crate::points::{Mean, point};

mod k_means;
mod points;

fn main() {
    let a = vec![point(1.0, 2.0)];
    let b = a.mean();
    println!("{:?}", b)
}
