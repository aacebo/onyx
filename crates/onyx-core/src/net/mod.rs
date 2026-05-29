pub mod asset;

pub use asset::Asset;

use crate::BoxFuture;

/// Represents a collection of assets
/// that can be queried.
pub trait Repository {
    type Error: std::error::Error;

    fn get(&self, path: &std::path::Path) -> BoxFuture<'_, Result<Asset, Self::Error>>;
    fn download(&self, src: &std::path::Path, dest: &std::path::Path) -> BoxFuture<'_, Result<u64, Self::Error>>;
}

/// Represents a collection of repositories
/// that can be used to query their assets.
pub trait DataSource {
    type Error: std::error::Error;
    type Target: Repository<Error = Self::Error>;

    fn load(&self, key: &str) -> BoxFuture<'_, Result<Self::Target, Self::Error>>;
}
