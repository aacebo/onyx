use std::fmt;

#[derive(Debug)]
pub enum ReadError {
    NotFound(String),
    Io(String),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(s) => write!(f, "not found: {s}"),
            Self::Io(s) => write!(f, "io: {s}"),
        }
    }
}

impl std::error::Error for ReadError {}
