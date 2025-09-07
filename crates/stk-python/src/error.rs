use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use stk_error::StkError;

create_exception!(error_module, PyBaseStkError, PyException);
create_exception!(error_module, PyStkParseError, PyBaseStkError);

pub enum PyStkError {
    Stk(StkError),
}

impl From<StkError> for PyStkError {
    fn from(err: StkError) -> Self {
        Self::Stk(err)
    }
}

impl From<PyStkError> for PyErr {
    fn from(err: PyStkError) -> Self {
        match err {
            PyStkError::Stk(StkError::Parse(msg)) => PyStkParseError::new_err(msg.to_string()),
        }
    }
}

pub fn into_pyerr(err: StkError) -> PyErr {
    PyStkError::from(err).into()
}

#[pymodule]
#[pyo3(name = "errors")]
pub fn error_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("StkError", m.py().get_type::<PyBaseStkError>())?;
    m.add("ParseError", m.py().get_type::<PyStkParseError>())?;
    Ok(())
}
