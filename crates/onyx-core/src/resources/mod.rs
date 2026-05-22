mod format;
mod location;

pub use format::*;
pub use location::*;

use crate::Error;

pub trait Decode: Sized {
    type Error: std::error::Error;

    fn decode(resource: &Resource) -> Result<Self, Self::Error>;
}

pub trait Resolver {
    type Error: std::error::Error;

    /// resolves a resource and returns
    /// its on-disk path.
    fn resolve(&self, location: &Location) -> impl Future<Output = Result<Resource, Self::Error>>;
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    pub path: std::path::PathBuf,
    pub format: Format,
    pub location: Location,
}

impl Resource {
    pub fn from_uri(uri: &str) -> Result<Self, Error> {
        let location = Location::parse(uri)?;
        Ok(Self::new(location))
    }

    pub fn new(location: Location) -> Self {
        let format = location.format();

        Self {
            path: std::env::temp_dir(),
            format,
            location,
        }
    }

    pub fn with_path(mut self, path: std::path::PathBuf) -> Self {
        self.path = path;
        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }
}
