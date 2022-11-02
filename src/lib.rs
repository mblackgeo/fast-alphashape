use delaunator::{triangulate, Point};
use itertools::Itertools;
use ndarray::*;
use ndarray_linalg::*;
use numpy::*;
use pyo3::prelude::*;
use std::collections::HashSet;

/// TODO docs
#[pyfunction]
fn alpha_shape_wrapper(points: PyReadonlyArray2<f64>, alpha: f64) -> PyResult<Vec<Vec<i32>>> {
    Ok(alpha_shape_edges(points.as_array(), alpha))
}

/// TODO docs
#[pymodule]
fn _fast_alphashape(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(alpha_shape_wrapper, m)?)?;
    Ok(())
}

/// Calculate the circumcentre of a set of points in barycentric coordinates.
pub fn circumcentre(points: ArrayView2<f64>) -> Array1<f64> {
    let n_rows = points.shape()[0];

    // Build the Coefficient matrix
    let matrix = concatenate![
        Axis(0),
        concatenate![
            Axis(1),
            2.0 * points.dot(&points.t()),
            Array::ones((n_rows, 1))
        ],
        concatenate![Axis(1), Array::ones((1, n_rows)), Array::zeros((1, 1))]
    ];

    // build the ordinate
    let ord = concatenate![
        Axis(0),
        (&points * &points).sum_axis(Axis(1)),
        Array::ones(1)
    ];

    // solve
    // TODO error handling here for failure to converge
    let res = matrix.solve_into(ord).unwrap();
    res.slice(s![..-1]).to_owned()
}

/// Calculate the circumradius of a given set of points
pub fn circumradius(points: ArrayView2<f64>) -> f64 {
    let slice = points.slice(s![0, ..]).to_owned();
    let centre = circumcentre(points.view());

    (slice - centre.dot(&points)).norm()
}

/// Returns simplices of the given set of points
pub fn alpha_simplices(points: ArrayView2<f64>) -> Vec<i32> {
    let pts: Vec<Point> = points
        .axis_iter(Axis(0))
        .map(|arr| Point {
            x: arr[0],
            y: arr[1],
        })
        .collect();

    triangulate(&pts)
        .triangles
        .iter()
        .map(|x| *x as i32)
        .collect()
}

// Return the indices of the array that form the edges of the 2D alpha shape
pub fn alpha_shape_edges(points: ArrayView2<f64>, alpha: f64) -> Vec<Vec<i32>> {
    // extract the simplex triangles
    let simplexes: Vec<i32> = alpha_simplices(points.view());

    // TODO separate this part out into a function
    // store the circumradius of each simplex and the indices of the row
    // of each point of the simplex
    let mut point_indices: Vec<Vec<i32>> = Vec::new();
    let mut radii: Vec<f64> = Vec::new();
    for tri in simplexes.chunks_exact(3) {
        let coords = stack![
            Axis(0),
            points.slice(s![tri[0], ..]),
            points.slice(s![tri[1], ..]),
            points.slice(s![tri[2], ..]),
        ];
        let rad = circumradius(coords.view());

        // extract the indices of each point of the simplex
        let mut idxs: Vec<i32> = Vec::new();
        for c in coords.rows() {
            for (idx, p) in points.rows().into_iter().enumerate() {
                if p == c {
                    idxs.push(idx as i32);
                }
            }
        }
        point_indices.push(idxs);
        radii.push(rad);
    }

    // Create a set to hold unique edges of simplices that pass the radius
    let mut edges: HashSet<Vec<i32>> = HashSet::new();
    for (point_idxs, radius) in point_indices.into_iter().zip(radii) {
        if radius < 1.0 / alpha {
            for edge in point_idxs.into_iter().combinations(2) {
                edges.insert(edge);
            }
        }
    }

    edges.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use ndarray::array;

    #[test]
    fn test_circumcentre() {
        let points = array![[1.0, 0.0], [0.5, 0.25], [0.0, 0.0]];
        let res = circumcentre(points.view());

        assert_relative_eq!(res[0], 1.25, epsilon = 1.0e-6);
        assert_relative_eq!(res[1], -1.5, epsilon = 1.0e-6);
        assert_relative_eq!(res[2], 1.25, epsilon = 1.0e-6);
    }

    #[test]
    fn test_circumradius() {
        let points = array![[1.0, 0.0], [0.5, 0.25], [0.0, 0.0]];
        let res = circumradius(points.view());

        assert_relative_eq!(res, 0.625, epsilon = 1.0e-6);
    }

    #[test]
    fn test_alpha_simplices() {
        let points = array![[1.0, 0.0], [0.5, 0.25], [0.0, 0.0]];
        let res = alpha_simplices(points.view());

        assert_eq!(res[0], 1);
        assert_eq!(res[1], 0);
        assert_eq!(res[2], 2);
    }

    #[test]
    fn test_alpha_shape_edges() {
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

        let mut expected = vec![
            [7, 4],
            [7, 3],
            [3, 4],
            [5, 4],
            [4, 0],
            [6, 1],
            [6, 5],
            [1, 5],
            [4, 5],
            [5, 7],
            [0, 6],
            [5, 2],
            [4, 6],
            [2, 7],
        ];

        let mut res = alpha_shape_edges(points.view(), alpha);

        assert_eq!(res.len(), expected.len());
        assert_eq!(res.sort(), expected.sort());
    }
}
