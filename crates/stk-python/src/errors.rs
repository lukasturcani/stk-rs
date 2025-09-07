use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use stk_error::StkError;

create_exception!(errors, PyBaseStkError, PyException);
create_exception!(errors, PyStkParseError, PyBaseStkError);

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
