mod format;
mod uri;

pub use format::*;
pub use uri::*;

use crate::Error;

pub trait Decoder {
    type Error: std::error::Error;

    fn decode<T>(resource: &Resource) -> Result<T, Self::Error>
    where
        T: for<'de> serde::Deserialize<'de>;
}

pub trait Resolver {
    type Error: std::error::Error;

    /// resolves a resource and returns
    /// its on-disk path.
    fn resolve(&self, uri: &Uri) -> impl Future<Output = Result<Resource, Self::Error>>;
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    pub path: Option<std::path::PathBuf>,
    pub format: Format,
    pub uri: Uri,
}

impl Resource {
    pub fn from_uri(uri: &str) -> Result<Self, Error> {
        let ur = Uri::parse(uri)?;
        Ok(Self::new(ur))
    }

    pub fn new(uri: Uri) -> Self {
        let format = uri.format();

        Self {
            path: uri.name().map(|v| std::env::temp_dir().join(v)),
            format,
            uri,
        }
    }

    pub fn with_directory(mut self, path: std::path::PathBuf) -> Self {
        self.path = self.uri.name().map(|v| path.join(v));
        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }
}
