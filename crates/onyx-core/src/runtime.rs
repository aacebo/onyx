use async_trait::async_trait;

use crate::{Error, model};

/// A loadable inference runtime.
#[async_trait]
pub trait Runtime: Send + Sync {
    /// The ready-to-run session this runtime produces.
    type Session: model::ModelSession;

    /// Load a model from `resource`, described by `manifest`, into a session.
    async fn load(&self, manifest: &model::ModelManifest) -> Result<Self::Session, Error>;
}
