use onyx_core::BoxFuture;
use onyx_core::error::ReadError;
use onyx_core::resource::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct TokioReader;

impl io::Reader for TokioReader {
    fn read<'a>(&'a self, resource: &'a onyx_core::Resource) -> BoxFuture<'a, onyx_core::error::Result<Vec<u8>>> {
        Box::pin(async move {
            if let Some(path) = &resource.path {
                Ok(tokio::fs::read(path).await?)
            } else if let Uri::Buffer(_, data) = &resource.uri {
                Ok(data.clone())
            } else {
                Err(ReadError::NotFound("file not found, no path provided".into()))?
            }
        })
    }
}
