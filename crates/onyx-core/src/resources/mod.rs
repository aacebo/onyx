mod format;
mod location;

pub use format::*;
pub use location::*;

use async_trait::async_trait;

pub trait Decode: Sized {
    type Error;

    fn decode(resource: &Resource) -> Result<Self, Self::Error>;
}

#[async_trait]
pub trait Resolver {
    type Error;

    /// resolves a resource and returns
    /// its on-disk path.
    async fn resolve(&self, uri: &str) -> Result<Resource, Self::Error>;
}

#[derive(Clone, PartialEq, Eq)]
pub struct Resource {
    pub name: String,
    pub format: ResourceFormat,
    pub location: ResourceLocation,
}
