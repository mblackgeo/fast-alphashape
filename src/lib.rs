#![allow(dead_code)]
use ndarray::*;
use ndarray_linalg::*;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _fast_alphashape(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
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
    let res = matrix.solve_into(ord).unwrap();
    res.slice(s![..-1]).to_owned()
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
}
