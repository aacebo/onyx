use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BufferResource {
    pub content: Arc<Vec<u8>>,
}
