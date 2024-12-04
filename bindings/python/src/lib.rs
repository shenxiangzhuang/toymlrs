
use toymlrs_clustering::kmeans::*;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


/// A Python module implemented in Rust.
#[pymodule]
fn _toymlrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    let _ = m.add_class::<Kmeans>();
    Ok(())
}