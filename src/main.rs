use prover::{math::fields::f64::BaseElement, Matrix, RowMatrix};
use rand_utils::rand_vector;
use std::time::Instant;
use utils::collections::Vec;

fn main() {
    let n = 1024;
    let num_polys = 64;
    let blowup_factor = 8;
    let columns: Vec<Vec<BaseElement>> = (0..num_polys).map(|_| rand_vector(n)).collect();
    let _row_matrix = RowMatrix::from_polys(&Matrix::new(columns), blowup_factor);
}
