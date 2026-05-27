mod format;
pub mod io;
pub mod net;
mod uri;

pub use format::*;
pub use uri::*;

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    pub path: Option<std::path::PathBuf>,
    pub format: Format,
    pub uri: Uri,
}

impl Resource {
    pub fn from_uri(uri: Uri) -> Self {
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
