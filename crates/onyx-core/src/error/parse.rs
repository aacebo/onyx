use std::fmt;

#[derive(Debug, Clone)]
pub enum ParseError {
    Empty,
    InvalidModelId(String),
    InvalidUri(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty input"),
            Self::InvalidModelId(s) => write!(f, "invalid model id: {s}"),
            Self::InvalidUri(s) => write!(f, "invalid uri: {s}"),
        }
    }
}

impl std::error::Error for ParseError {}
