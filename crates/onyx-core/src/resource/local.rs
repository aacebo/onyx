use async_trait::async_trait;

use crate::{Resource, error};

#[derive(Clone, PartialEq, Eq)]
pub struct LocalResource {
    name: String,
    path: std::path::PathBuf,
}

impl LocalResource {
    pub fn parse(url: impl AsRef<str>) -> Result<Self, error::ResourceError> {
        use std::str::FromStr;

        let rest = url.as_ref().trim_start_matches("file://");
        let path = std::path::PathBuf::from_str(rest).map_err(error::ResourceError::parse)?;
        let name = path
            .file_name()
            .ok_or(error::ResourceError::parse(
                "invalid file url -> filename is required",
            ))?
            .to_str()
            .ok_or(error::ResourceError::parse(
                "invalid file url -> filename must be unicode",
            ))?
            .to_string();

        Ok(Self { name, path })
    }
}

impl LocalResource {
    pub fn filename(&self) -> &str {
        &self.name
    }
}

impl TryFrom<std::path::PathBuf> for LocalResource {
    type Error = error::ResourceError;

    fn try_from(path: std::path::PathBuf) -> Result<Self, Self::Error> {
        let name = path
            .file_name()
            .ok_or(error::ResourceError::parse(
                "invalid file url -> filename is required",
            ))?
            .to_str()
            .ok_or(error::ResourceError::parse(
                "invalid file url -> filename must be unicode",
            ))?
            .to_string();

        Ok(Self { name, path })
    }
}

impl std::ops::Deref for LocalResource {
    type Target = std::path::PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl std::str::FromStr for LocalResource {
    type Err = error::ResourceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl std::fmt::Debug for LocalResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl serde::Serialize for LocalResource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for LocalResource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for LocalResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "file://{}", self.path.as_path().display())
    }
}

#[async_trait]
impl Resource for LocalResource {
    async fn read(&self) -> Result<std::path::PathBuf, error::ResourceError> {
        Ok(self.path.clone())
    }
}
