#[derive(Debug)]
pub struct Directory {
    meta: DirectoryMeta,
    items: Vec<super::AssetMeta>,
}

impl Directory {
    pub fn meta(&self) -> &DirectoryMeta {
        &self.meta
    }

    pub fn items(&self) -> &[super::AssetMeta] {
        &self.items
    }
}

impl std::ops::Deref for Directory {
    type Target = DirectoryMeta;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DirectoryMeta {
    path: std::path::PathBuf,
    name: String,
}

impl DirectoryMeta {
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
