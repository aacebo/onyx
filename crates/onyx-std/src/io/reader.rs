use onyx_core::error::ReadError;
use onyx_core::resource::{Reader, Uri};
use onyx_core::{BoxFuture, Resource};

#[derive(Default, Copy, Clone)]
pub struct StdReader;

impl Reader for StdReader {
    fn read<'a>(&'a self, resource: &'a Resource) -> BoxFuture<'a, onyx_core::error::Result<Vec<u8>>> {
        Box::pin(async move {
            if let Some(path) = &resource.path {
                Ok(std::fs::read(path)?)
            } else if let Uri::Buffer(bytes) = &resource.uri {
                Ok(bytes.data.clone())
            } else {
                Err(ReadError::NotFound("file not found, no path provided".into()))?
            }
        })
    }
}
