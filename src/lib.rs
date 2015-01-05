#![feature(slicing_syntax)]
use std::fmt::Show;
use std::mem::size_of;
use std::iter::IteratorExt;
use std::ops::Mul;
use std::ops::Add;

#[derive(Show, PartialEq, Clone)]
pub struct SMat<T> {
    primary_index: Vec<uint>,
    secondary_index: Vec<uint>,
    data: Vec<T>
}

impl<T: Clone + Ord> SMat<T> {
    pub fn new(pairs: &mut [(uint, uint, T)]) -> SMat<T> {
        pairs.sort();
        let size_hint = pairs.len();
        let mut primary_index = Vec::with_capacity(size_hint);
        primary_index.push(0); // Set up first element to be 0.
        let mut secondary_index = Vec::with_capacity(size_hint);
        let mut data = Vec::with_capacity(size_hint);

        let mut row = 0;
        for &(row_index, column_index, ref elem) in pairs.iter() {
            while row_index != row {
                primary_index.push(secondary_index.len());
                row += 1;
            }
            secondary_index.push(column_index);
            data.push(elem.clone())
        }

        SMat {
            primary_index: primary_index,
            secondary_index: secondary_index,
            data: data
        }
    }
}

// TODO: use semiring
fn mult<T: Mul, Add>(a: SMat<T>, b: Vec<T>) -> Vec<T> {
    let mut product: Vec<T> = Vec::with_capacity(b.len());
    for r in range(0, b.len()) {
        // we iterate through indicies until we hit the end of the row
        let row = a.primary_index[r];
        let next_row = if r == b.len() - 1 { a.secondary_index.len() } else { a.primary_index[r+1] };

        product.push(range(row, next_row).fold(0, |acc, ind| {
            acc += a.data[ind] * b[a.secondary_index[ind]]
        }));
    }
    product
}
