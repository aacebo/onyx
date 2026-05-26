use std::fmt;

#[derive(Debug)]
pub enum UnsupportedError {
    Backend(String),
}

impl fmt::Display for UnsupportedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backend(s) => write!(f, "backend: {s}"),
        }
    }
}

impl std::error::Error for UnsupportedError {}
