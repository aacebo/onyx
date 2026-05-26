use std::fmt;

#[derive(Debug)]
pub enum ResolveError {
    NotFound(String),
    Unavailable(String),
    UnsupportedScheme(String),
    PermissionDenied(String),
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(s) => write!(f, "not found: {s}"),
            Self::Unavailable(s) => write!(f, "unavailable: {s}"),
            Self::UnsupportedScheme(s) => write!(f, "unsupported scheme: {s}"),
            Self::PermissionDenied(s) => write!(f, "permission denied: {s}"),
        }
    }
}

impl std::error::Error for ResolveError {}
