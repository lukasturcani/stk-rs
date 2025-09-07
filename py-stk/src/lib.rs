use pyo3::prelude::*;
use pyo3::py_run;
use stk_python::errors::{PyBaseStkError, PyStkParseError};
use stk_python::molecular_graph::PyMolecularGraph;

/// A Python module implemented in Rust.
#[pymodule]
fn stk(py: Python<'_>, _: &Bound<'_, PyModule>) -> PyResult<()> {
    setup_molecular_graph(py)?;
    setup_errors(py)?;
    Ok(())
}

fn setup_molecular_graph(py: Python<'_>) -> PyResult<()> {
    let molecular_graph = PyModule::new(py, "stk.molecular_graph")?;
    molecular_graph.add_class::<PyMolecularGraph>()?;
    py_run!(
        py,
        molecular_graph,
        "import sys; sys.modules['stk.molecular_graph'] = molecular_graph"
    );
    Ok(())
}

fn setup_errors(py: Python<'_>) -> PyResult<()> {
    let errors = PyModule::new(py, "stk.errors")?;
    errors.add("StkError", errors.py().get_type::<PyBaseStkError>())?;
    errors.add("ParseError", errors.py().get_type::<PyStkParseError>())?;
    Ok(())
}
