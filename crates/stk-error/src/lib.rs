use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorString(Cow<'static, str>);

impl ErrorString {
    pub const fn new_static(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }
}

impl<T> From<T> for ErrorString
where
    T: Into<Cow<'static, str>>,
{
    fn from(msg: T) -> Self {
        ErrorString(msg.into())
    }
}

impl AsRef<str> for ErrorString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ErrorString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ErrorString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum StkError {
    Parse(ErrorString),
}

impl Display for StkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StkError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}
