use crate::{BoxFuture, fs};

pub trait Download {
    type Error: std::error::Error;

    fn download(&self, url: &url::Url, dest: &dyn fs::File) -> BoxFuture<'_, Result<u64, Self::Error>>;
}
