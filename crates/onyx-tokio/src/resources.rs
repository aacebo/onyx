use onyx_core::resources;

#[derive(Default)]
pub struct TokioResourceResolver {
    path: Option<std::path::PathBuf>,
}

impl TokioResourceResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_path(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl resources::Resolver for TokioResourceResolver {
    type Error = tokio::io::Error;

    async fn resolve(
        &self,
        location: &resources::Location,
    ) -> Result<resources::Resource, Self::Error> {
        let mut resource = resources::Resource::new(location.clone());

        if let Some(path) = &self.path {
            resource = resource.with_path(path.clone());
        }

        if tokio::fs::try_exists(&resource.path).await? {
            return Ok(resource);
        }

        match &resource.location {
            resources::Location::Local(path) => {
                tokio::fs::copy(path, &resource.path).await?;
                Ok(resource)
            }
            resources::Location::Buffer(_, data) => {
                tokio::fs::write(&resource.path, data).await?;
                Ok(resource)
            }
            #[cfg(feature = "http")]
            resources::Location::Http(url) => {
                let mut res = reqwest::get(url.as_str())
                    .await
                    .map_err(|err| tokio::io::Error::other(err))?
                    .error_for_status()
                    .map_err(|err| tokio::io::Error::other(err))?;

                let mut file = tokio::fs::File::create(&resource.path).await?;

                while let Some(mut chunk) = res
                    .chunk()
                    .await
                    .map_err(|err| tokio::io::Error::other(err))?
                {
                    use tokio::io::AsyncWriteExt;

                    file.write_all_buf(&mut chunk).await?;
                }

                Ok(resource)
            }
        }
    }
}
