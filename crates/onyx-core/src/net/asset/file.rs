use std::{cell::LazyCell, sync::LazyLock};

use crate::BoxAsyncFuture;

pub struct File {
    meta: FileMeta,
    factory: LazyLock<Vec<u8>, BoxAsyncFuture<Vec<u8>>>,
}

impl File {
    pub fn new<F>(meta: FileMeta, factory: F) -> Self
    where
        F: FnOnce() -> std::pin::Pin<Box<dyn Future<Output = Vec<u8>>>> + 'static,
    {
        Self {
            meta,
            factory: LazyLock::new(Box::new(async || factory())),
        }
    }

    pub fn meta(&self) -> &FileMeta {
        &self.meta
    }

    pub async fn read(&self) -> Vec<u8> {
        (self.factory)().await
    }
}

impl std::ops::Deref for File {
    type Target = FileMeta;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FileMeta {
    path: std::path::PathBuf,
    name: String,
    extension: Option<String>,
}

impl FileMeta {
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn extension(&self) -> Option<&str> {
        self.extension.as_deref()
    }
}
