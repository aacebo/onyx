mod bytes;
mod format;
pub mod io;
pub mod net;
mod uri;

pub use bytes::*;
pub use format::*;
pub use uri::*;

pub trait Artifact {
    fn name(&self) -> &str;
    fn format(&self) -> Format;
    fn source(&self) -> &Uri;
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    pub path: Option<std::path::PathBuf>,
    pub uri: Uri,
    pub format: Format,
}

impl Resource {
    pub fn from_uri(uri: Uri) -> Self {
        let format = uri.format();

        Self {
            path: uri.name().map(|v| std::env::temp_dir().join(v)),
            uri,
            format,
        }
    }
}

impl Artifact for Resource {
    fn name(&self) -> &str {
        self.uri.name().unwrap_or("??")
    }

    fn format(&self) -> Format {
        self.format
    }

    fn source(&self) -> &Uri {
        &self.uri
    }
}
