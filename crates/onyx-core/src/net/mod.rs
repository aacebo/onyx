pub mod asset;

pub use asset::{Asset, AssetData};

use crate::BoxFuture;

/// Represents a collection of assets
/// that can be queried.
pub trait Repository: Send + Sync {
    fn exists(&self, path: &std::path::Path) -> BoxFuture<'_, crate::Result<bool>>;
    fn get(&self, path: &std::path::Path) -> BoxFuture<'_, crate::Result<Asset>>;
    fn read(&self, path: &std::path::Path) -> BoxFuture<'_, crate::Result<AssetData>>;
    fn copy(&self, src: &std::path::Path, dest: &std::path::Path) -> BoxFuture<'_, crate::Result<u64>>;
}

/// Represents a collection of repositories
/// that can be used to query their assets.
pub trait DataSource: Send + Sync {
    fn load(&self, key: &str) -> BoxFuture<'_, crate::Result<std::sync::Arc<dyn Repository>>>;
}
