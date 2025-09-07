use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn stk(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_submodule();
    Ok(())
}

#[pymodule]
fn molecular_graph(m: &PyModule) -> PyResult<()> {
    Ok(())
}
