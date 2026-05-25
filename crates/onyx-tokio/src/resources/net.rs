use onyx_core::resources::*;

#[derive(Debug, Default, Clone)]
pub struct TokioResolver {
    dir: Option<std::path::PathBuf>,
}

impl TokioResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_directory(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.dir = Some(path.into());
        self
    }
}

impl net::Resolver for TokioResolver {
    type Error = tokio::io::Error;

    async fn resolve(&self, uri: &Uri) -> Result<Resource, Self::Error> {
        let mut resource = Resource::from_uri(uri.clone());

        if let Some(path) = &self.dir {
            resource = resource.with_directory(path.clone());
        }

        let path = if let Some(p) = &resource.path {
            p
        } else {
            return Ok(resource);
        };

        if tokio::fs::try_exists(&path).await? {
            return Ok(resource);
        }

        match &resource.uri {
            Uri::Local(src) => {
                tokio::fs::copy(src, path).await?;
                Ok(resource)
            }
            Uri::Buffer(_, data) => {
                tokio::fs::write(&path, data).await?;
                Ok(resource)
            }
            #[cfg(feature = "http")]
            Uri::Http(url) => {
                let mut res = reqwest::get(url.as_str())
                    .await
                    .map_err(|err| tokio::io::Error::other(err))?
                    .error_for_status()
                    .map_err(|err| tokio::io::Error::other(err))?;

                let mut file = tokio::fs::File::create(path).await?;

                while let Some(mut chunk) = res.chunk().await.map_err(|err| tokio::io::Error::other(err))? {
                    use tokio::io::AsyncWriteExt;

                    file.write_all_buf(&mut chunk).await?;
                }

                Ok(resource)
            }
        }
    }
}
