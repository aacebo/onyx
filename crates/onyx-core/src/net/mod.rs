pub mod asset;

pub use asset::Asset;

use crate::BoxFuture;

pub trait Repository {
    type Error;

    fn get(&self, path: &std::path::Path) -> BoxFuture<'_, Result<Asset, Self::Error>>;
    fn download(&self, src: &std::path::Path, dest: &std::path::Path) -> BoxFuture<'_, Result<u64, Self::Error>>;
}
