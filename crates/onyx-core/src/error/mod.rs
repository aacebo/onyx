mod model;
mod resource;

pub use model::*;
pub use resource::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Error {
    Model(ModelError),
    Resource(ResourceError),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Model(v) => Some(v),
            Self::Resource(v) => Some(v),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Model(v) => write!(f, "{v}"),
            Self::Resource(v) => write!(f, "{v}"),
        }
    }
}

impl From<ModelError> for Error {
    fn from(value: ModelError) -> Self {
        Self::Model(value)
    }
}

impl From<ResourceError> for Error {
    fn from(value: ResourceError) -> Self {
        Self::Resource(value)
    }
}
