mod directory;
mod file;
mod remote;

pub use directory::*;
pub use file::*;
pub use remote::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Asset {
    File(File),
    Directory(Directory),
    Remote(Remote),
}

impl Asset {
    pub fn path(&self) -> &std::path::Path {
        match self {
            Self::File(v) => v.path(),
            Self::Directory(v) => v.path(),
            Self::Remote(v) => v.path(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::File(v) => v.name(),
            Self::Directory(v) => v.name(),
            Self::Remote(v) => v.name(),
        }
    }

    pub fn extension(&self) -> Option<&str> {
        match self {
            Self::File(v) => v.extension(),
            _ => None,
        }
    }
}

impl From<File> for Asset {
    fn from(value: File) -> Self {
        Self::File(value)
    }
}

impl From<Directory> for Asset {
    fn from(value: Directory) -> Self {
        Self::Directory(value)
    }
}

impl From<Remote> for Asset {
    fn from(value: Remote) -> Self {
        Self::Remote(value)
    }
}
