use pyo3::prelude::*;
use pyo3::py_run;
use pyo3::wrap_pymodule;
use stk_python::errors::errors;
use stk_python::molecular_graph::PyMolecularGraph;

/// A Python module implemented in Rust.
#[pymodule]
fn stk(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    molecular_graph(py)?;
    m.add_wrapped(wrap_pymodule!(errors))?;
    Ok(())
}

fn molecular_graph(py: Python<'_>) -> PyResult<()> {
    let m = PyModule::new(py, "molecular_graph")?;
    m.add_class::<PyMolecularGraph>()?;
    py_run!(
        py,
        m,
        "import sys; sys.modules['stk.molecular_graph'] = molecular_graph"
    );
    Ok(())
}
