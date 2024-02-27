use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustify(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum, m)?)?;
    Ok(())
}
