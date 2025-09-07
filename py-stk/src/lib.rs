use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use stk_python::error::error_module;
use stk_python::molecular_graph::PyMolecularGraph;

/// A Python module implemented in Rust.
#[pymodule]
fn stk(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(molecular_graph))?;
    Ok(())
}

#[pymodule]
fn molecular_graph(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMolecularGraph>()?;
    Ok(())
}
