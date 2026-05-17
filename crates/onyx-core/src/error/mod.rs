mod resource;
mod runtime;

pub use resource::*;
pub use runtime::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Error {
    Resource(ResourceError),
    Runtime(RuntimeError),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Resource(v) => Some(v),
            Self::Runtime(v) => Some(v),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Resource(v) => write!(f, "{}", v),
            Self::Runtime(v) => write!(f, "{}", v),
        }
    }
}

impl From<ResourceError> for Error {
    fn from(value: ResourceError) -> Self {
        Self::Resource(value)
    }
}

impl From<RuntimeError> for Error {
    fn from(value: RuntimeError) -> Self {
        Self::Runtime(value)
    }
}
