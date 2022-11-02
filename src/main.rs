extern crate ndarray;

use _fast_alphashape::*;
use ndarray::*;

fn main() {
    let points = array![
        [0., 0.],
        [0., 1.],
        [1., 1.],
        [1., 0.],
        [0.5, 0.25],
        [0.5, 0.75],
        [0.25, 0.5],
        [0.75, 0.5]
    ];
    let alpha = 2.0;

    let res = alpha_shape_edges(points.view(), alpha);
    println!("{:?}", res);
    println!("{:?}", res.len());
}
