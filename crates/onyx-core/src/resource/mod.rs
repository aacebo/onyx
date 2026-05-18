#[cfg(feature = "huggingface")]
mod huggingface;
mod local;
mod remote;

#[cfg(feature = "huggingface")]
pub use huggingface::*;
pub use local::*;
pub use remote::*;

use async_trait::async_trait;

use crate::error;

#[async_trait]
pub trait Resource {
    async fn read(&self) -> Result<std::path::PathBuf, error::ResourceError>;
}
