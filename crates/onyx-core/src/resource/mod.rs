mod bytes;
mod format;
mod uri;

pub use bytes::*;
pub use format::*;
pub use uri::*;

pub trait Reader: Send + Sync {
    fn read<'a>(&'a self, resource: &'a Resource) -> crate::BoxFuture<'a, crate::error::Result<Vec<u8>>>;
}

pub trait Resolver: Send + Sync {
    /// resolves a resource and returns
    /// its on-disk path.
    fn resolve<'a>(&'a self, uri: &'a Uri) -> crate::BoxFuture<'a, crate::error::Result<Resource>>;
}

pub trait Artifact {
    fn name(&self) -> &str;
    fn uri(&self) -> &Uri;
    fn format(&self) -> Format;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    pub path: Option<std::path::PathBuf>,
    pub uri: Uri,
    pub size: u64,
    pub format: Format,
}

impl Resource {
    pub fn from_uri(uri: Uri) -> Self {
        let format = uri.format();

        Self {
            path: uri.name().map(|v| std::env::temp_dir().join(v)),
            uri,
            size: 0,
            format,
        }
    }
}

impl Artifact for Resource {
    fn name(&self) -> &str {
        self.uri.name().unwrap_or("??")
    }

    fn uri(&self) -> &Uri {
        &self.uri
    }

    fn format(&self) -> Format {
        self.format
    }
}
