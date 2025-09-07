use stk_error::StkError;

pub enum PyStkError {
    Stk(StkError),
}
