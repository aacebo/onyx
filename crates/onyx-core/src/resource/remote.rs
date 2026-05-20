use std::{collections::HashMap, io::Write};

use async_trait::async_trait;

use crate::{Resource, error};

#[derive(Clone, PartialEq, Eq)]
pub struct RemoteResource {
    name: String,
    url: reqwest::Url,
    path: std::path::PathBuf,
}

impl RemoteResource {
    pub fn parse(url: impl AsRef<str>) -> Result<Self, error::ResourceError> {
        use std::str::FromStr;

        let url = reqwest::Url::parse(url.as_ref()).map_err(error::ResourceError::parse)?;
        let name = url
            .path_segments()
            .ok_or(error::ResourceError::parse("invalid resource url"))?
            .filter(|s| !s.is_empty())
            .last()
            .ok_or(error::ResourceError::parse("invalid resource url"))?
            .to_string();

        let query: HashMap<String, String> = url.query_pairs().into_owned().collect();
        let mut path = std::env::temp_dir().join(format!("onyx/{}", &name));

        if let Some(param) = query.get("path") {
            path = std::path::PathBuf::from_str(param.as_str())
                .map_err(error::ResourceError::parse)?;
            path = path.join(&name);
        }

        Ok(Self { url, name, path })
    }
}

impl RemoteResource {
    pub fn filename(&self) -> &str {
        &self.name
    }
}

impl std::str::FromStr for RemoteResource {
    type Err = error::ResourceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl std::fmt::Debug for RemoteResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for RemoteResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl serde::Serialize for RemoteResource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for RemoteResource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[async_trait]
impl Resource for RemoteResource {
    async fn read(&self) -> Result<std::path::PathBuf, error::ResourceError> {
        if std::fs::exists(&self.path).map_err(error::ResourceError::io)? {
            return Ok(self.path.clone());
        }

        let mut res = reqwest::get(self.url.as_str())
            .await
            .map_err(error::ResourceError::api)?;

        let mut file = std::fs::File::create(&self.path).map_err(error::ResourceError::io)?;

        while let Some(chunk) = res.chunk().await.map_err(error::ResourceError::api)? {
            file.write_all(&chunk).map_err(error::ResourceError::io)?;
        }

        Ok(self.path.clone())
    }
}
