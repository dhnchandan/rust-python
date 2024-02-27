use pyo3::prelude::*;

#[pyclass]
struct Product {
    title: String,
    stock: i16,
    price: f32
}

#[pymethods]
impl Product {
    #[new]
    fn new(title: String, stock: i16, price: f32) -> Self {
        Product { title, stock, price }
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustify(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum, m)?)?;
    m.add_class::<Product>()?;
    Ok(())
}
