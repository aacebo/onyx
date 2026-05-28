use super::{Resource, Uri};
use crate::BoxFuture;
use crate::error::ReadError;

pub trait Reader: Send + Sync {
    fn read<'a>(&'a self, resource: &'a Resource) -> BoxFuture<'a, crate::error::Result<Vec<u8>>>;
}

#[derive(Default, Copy, Clone)]
pub struct StdReader;

impl Reader for StdReader {
    fn read<'a>(&'a self, resource: &'a Resource) -> BoxFuture<'a, crate::error::Result<Vec<u8>>> {
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
