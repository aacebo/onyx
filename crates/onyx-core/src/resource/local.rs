#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocalResource {
    pub path: std::path::PathBuf,
}
