use onyx_core::resources;

pub struct TokioResourceResolver;

impl resources::Resolver for TokioResourceResolver {
    type Error = tokio::io::Error;

    async fn resolve(
        &self,
        location: &resources::Location,
    ) -> Result<resources::Resource, Self::Error> {
        let resource = resources::Resource::new(location.clone());

        match &resource.location {
            resources::Location::Local(path) => {
                if tokio::fs::try_exists(&resource.path).await? {
                    return Ok(resource);
                }

                tokio::fs::copy(path, &resource.path).await?;
                Ok(resource)
            }
            #[cfg(feature = "http")]
            resources::Location::Http(url) => {
                if tokio::fs::try_exists(&resource.path).await? {
                    return Ok(resource);
                }

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
            _ => Ok(resource),
        }
    }
}
