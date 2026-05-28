mod bytes;
mod format;
mod io;
mod net;
mod uri;

pub use bytes::*;
pub use format::*;
pub use io::*;
pub use net::*;
pub use uri::*;

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
