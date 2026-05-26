use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    MissingField(&'static str),
    InvalidField(&'static str),
    UnsupportedArchitecture(String),
    UnsupportedModelType(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField(s) => write!(f, "missing field: {s}"),
            Self::InvalidField(s) => write!(f, "invalid field: {s}"),
            Self::UnsupportedArchitecture(s) => write!(f, "unsupported architecture: {s}"),
            Self::UnsupportedModelType(s) => write!(f, "unsupported model type: {s}"),
        }
    }
}

impl std::error::Error for ConfigError {}
