mod local;
mod remote;

pub use local::*;
pub use remote::*;

use async_trait::async_trait;

use crate::{error, models};

#[async_trait]
pub trait Resource {
    async fn download(&self) -> Result<std::path::PathBuf, error::ResourceError>;

    async fn read(&self) -> Result<Vec<u8>, error::ResourceError> {
        let path = self.download().await?;
        Ok(std::fs::read(path).map_err(error::ResourceError::io)?)
    }
}
