#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Directory {
    path: std::path::PathBuf,
    name: String,
}

impl Directory {
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
