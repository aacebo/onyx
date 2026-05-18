use async_trait::async_trait;

use crate::{Resource, error};

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct LocalResource(std::path::PathBuf);

impl LocalResource {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        Self(path.into())
    }
}

impl From<std::path::PathBuf> for LocalResource {
    fn from(value: std::path::PathBuf) -> Self {
        Self::new(value)
    }
}

impl std::ops::Deref for LocalResource {
    type Target = std::path::PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for LocalResource {
    type Err = error::ResourceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let rest = value.trim_start_matches("file://");

        Ok(Self(
            std::path::PathBuf::from_str(rest).map_err(error::ResourceError::parse)?,
        ))
    }
}

impl std::fmt::Debug for LocalResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for LocalResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "file://{}", self.0.as_path().display())
    }
}

#[async_trait]
impl Resource for LocalResource {
    async fn read(&self) -> Result<std::path::PathBuf, error::ResourceError> {
        Ok(self.0.clone())
    }
}
