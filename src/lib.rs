#![allow(dead_code)]
use ndarray::Array2;
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
fn circumcentre(_coords: Array2<f64>) -> Array2<f64> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_circumcentre() {
        let input = array![[1.0, 0.0], [0.5, 0.25], [0.0, 0.0]];
        let res = circumcentre(input);

        // assert_relative_eq!(res[0], 1.25, epsilon = 1.0e-6)
        // assert_relative_eq!(res[1], -1.5, epsilon = 1.0e-6)
        // assert_relative_eq!(res[2], -1.25, epsilon = 1.0e-6)
    }
}
