use std::fmt;

#[derive(Debug)]
pub enum LoadError {
    MissingArtifact(String),
    InvalidWeights(String),
    InvalidConfig(String),
    Backend(String),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingArtifact(s) => write!(f, "missing artifact: {s}"),
            Self::InvalidWeights(s) => write!(f, "invalid weights: {s}"),
            Self::InvalidConfig(s) => write!(f, "invalid config: {s}"),
            Self::Backend(s) => write!(f, "backend: {s}"),
        }
    }
}

impl std::error::Error for LoadError {}
