#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct File {
    path: std::path::PathBuf,
    name: String,
    extension: Option<String>,
}

impl File {
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn extension(&self) -> Option<&str> {
        self.extension.as_deref()
    }
}
