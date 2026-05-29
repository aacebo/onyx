mod directory;
mod file;

pub use directory::*;
pub use file::*;

#[derive(Debug)]
pub enum Asset {
    File(File),
    Directory(Directory),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AssetMeta {
    File(FileMeta),
    Directory(DirectoryMeta),
}

impl AssetMeta {
    pub fn path(&self) -> &std::path::Path {
        match self {
            Self::File(v) => v.path(),
            Self::Directory(v) => v.path(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::File(v) => v.name(),
            Self::Directory(v) => v.name(),
        }
    }
}

impl From<FileMeta> for AssetMeta {
    fn from(value: FileMeta) -> Self {
        Self::File(value)
    }
}

impl From<DirectoryMeta> for AssetMeta {
    fn from(value: DirectoryMeta) -> Self {
        Self::Directory(value)
    }
}
