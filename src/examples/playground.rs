extern crate matrix;

use matrix::SMat;

fn main() {
    let mut data = vec![(0,0,2),(0,1,1),(1,0,2),(3,1,3)];
    let smat: SMat<int> = SMat::new(data.as_mut_slice());
    println!("Matrix: {}", smat);
}
