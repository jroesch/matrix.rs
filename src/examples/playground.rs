extern crate matrix;

use matrix::SMat;

fn main() {
    let mut data = Vec::new();
    let smat: SMat<int> = SMat::new(data.as_mut_slice());
    println!("Matrix: {}", smat);
}
