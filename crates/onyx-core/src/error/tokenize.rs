use std::fmt;

#[derive(Debug)]
pub enum TokenizeError {
    InvalidInput(String),
    Backend(String),
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(s) => write!(f, "invalid input: {s}"),
            Self::Backend(s) => write!(f, "backend: {s}"),
        }
    }
}

impl std::error::Error for TokenizeError {}
