use std::error::Error;
use stk_error::StkError;

pub enum PyStkError {
    Stk(StkError),
}
