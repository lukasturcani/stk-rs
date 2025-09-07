use pyo3::prelude::*;
use pyo3::wrap_pymodule;

/// A Python module implemented in Rust.
#[pymodule]
fn stk(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(molecular_graph))?;
    Ok(())
}

#[pymodule]
fn molecular_graph(_: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
