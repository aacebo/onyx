mod architecture;
mod feature;

pub use architecture::*;
pub use feature::*;

use async_trait::async_trait;

use crate::{Error, tensor};

/// A loaded model, ready to execute forward passes.
#[async_trait]
pub trait ModelSession: Send + Sync {
    /// The manifest the session was loaded from.
    fn manifest(&self) -> &ModelManifest;

    /// Run one forward pass over the named input tensors.
    async fn infer(&self, inputs: tensor::TensorMap) -> Result<tensor::TensorMap, Error>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelManifest {
    pub name: String,
    pub architecture: ModelArchitecture,
    pub features: Vec<ModelFeature>,
    pub inputs: tensor::TensorSchema,
    pub outputs: tensor::TensorSchema,
}
