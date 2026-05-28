use onyx_core::BoxFuture;
use onyx_core::resource::*;

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

impl Resolver for TokioResolver {
    fn resolve<'a>(&'a self, uri: &'a Uri) -> BoxFuture<'a, onyx_core::error::Result<Resource>> {
        Box::pin(async move {
            let mut resource = Resource::from_uri(uri.clone());

            if let Some(path) = &self.dir {
                resource.path = resource.uri.name().map(|name| path.join(name));
            }

            let path = if let Some(p) = &resource.path {
                p
            } else {
                return Ok(resource);
            };

            if let Ok(meta) = tokio::fs::metadata(path).await {
                resource.size = meta.len();
            }

            if tokio::fs::try_exists(&path).await? {
                return Ok(resource);
            }

            match &resource.uri {
                Uri::Local(src) => {
                    crate::fs::symlink(src, path).await?;
                    Ok(resource)
                }
                Uri::Buffer(bytes) => {
                    tokio::fs::write(&path, &bytes.data).await?;
                    Ok(resource)
                }
                Uri::Http(url) => {
                    let mut res = reqwest::get(url.as_str())
                        .await
                        .map_err(|err| onyx_core::error::ResolveError::Unavailable(err.to_string()))?
                        .error_for_status()
                        .map_err(|err| onyx_core::error::ResolveError::Unavailable(err.to_string()))?;

                    let mut file = tokio::fs::File::create(path).await?;

                    while let Some(mut chunk) = res
                        .chunk()
                        .await
                        .map_err(|err| onyx_core::error::ResolveError::Unavailable(err.to_string()))?
                    {
                        use tokio::io::AsyncWriteExt;

                        file.write_all_buf(&mut chunk).await?;
                    }

                    Ok(resource)
                }
            }
        })
    }
}
