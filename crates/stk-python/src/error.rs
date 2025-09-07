use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::error::Error;
use stk_error::StkError;

create_exception!(error_module, PyStkError, PyException);
create_exception!(error_module, PyStkParseError, PyStkError);


impl From<

#[pymodule]
fn error_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("StkError", m.py().get_type::<PyStkError>())?;
    m.add("ParseError", m.py().get_type::<PyStkParseError>())?;
    Ok(())
}
