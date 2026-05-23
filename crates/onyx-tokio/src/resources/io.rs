use onyx_core::resources::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct TokioReader;

impl io::Reader for TokioReader {
    type Error = tokio::io::Error;

    async fn read(&self, resource: &onyx_core::Resource) -> Result<Vec<u8>, Self::Error> {
        if let Some(path) = &resource.path {
            tokio::fs::read(path).await
        } else if let Uri::Buffer(_, data) = &resource.uri {
            Ok(data.clone())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file not found, no path provided",
            ))
        }
    }
}
