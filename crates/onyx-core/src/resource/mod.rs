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
}
