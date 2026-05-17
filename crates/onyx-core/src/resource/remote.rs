#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteResource {
    pub url: String,
    pub path: std::path::PathBuf,
}
