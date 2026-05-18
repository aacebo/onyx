use std::io::Write;

use async_trait::async_trait;

use crate::{Resource, error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteResource {
    url: String,
    cache: std::path::PathBuf,
}

impl RemoteResource {
    pub fn new(url: impl Into<String>, cache: impl Into<std::path::PathBuf>) -> Self {
        Self {
            url: url.into(),
            cache: cache.into(),
        }
    }
}

#[async_trait]
impl Resource for RemoteResource {
    async fn read(&self) -> Result<std::path::PathBuf, error::ResourceError> {
        if std::fs::exists(&self.cache).map_err(error::ResourceError::io)? {
            return Ok(self.cache.clone());
        }

        let mut res = reqwest::get(&self.url)
            .await
            .map_err(error::ResourceError::api)?;

        let mut file = std::fs::File::create(&self.cache).map_err(error::ResourceError::io)?;

        while let Some(chunk) = res.chunk().await.map_err(error::ResourceError::api)? {
            file.write_all(&chunk).map_err(error::ResourceError::io)?;
        }

        Ok(self.cache.clone())
    }
}
