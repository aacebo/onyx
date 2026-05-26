use std::fmt;

#[derive(Debug)]
pub enum DecodeError {
    InvalidFormat(String),
    Json(String),
    Binary(String),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(s) => write!(f, "invalid format: {s}"),
            Self::Json(s) => write!(f, "json: {s}"),
            Self::Binary(s) => write!(f, "binary: {s}"),
        }
    }
}

impl std::error::Error for DecodeError {}
