//! Load/session abstraction implemented by peer backend crates (e.g. `onyx-ort`).
//!
//! The contract is layered: a [`Runtime`] loads a model described by a
//! [`crate::model::ModelManifest`] from a [`crate::resource::Resource`] into a
//! [`Session`], and the session runs typed forward passes over named tensors.
//!
//! ## Async & dyn-compatibility
//!
//! [`Runtime::load`] and [`Session::infer`] are `async fn`s exposed via
//! [`#[async_trait]`](async_trait::async_trait). Their futures are `Send` and
//! the traits are `dyn`-compatible, so backends can be used both with static
//! dispatch (`fn run<R: Runtime>(r: &R)`) and as trait objects
//! (`Arc<dyn Session>`, `tokio::spawn(...)`).

use async_trait::async_trait;

use crate::error::Error;
use crate::io::{Inputs, Outputs};
use crate::model::ModelManifest;
use crate::resource::Resource;
use crate::tensor::{DType, Shape};

/// A loadable inference runtime.
///
/// Implemented by peer crates that wrap a concrete inference engine
/// (ONNX Runtime, etc.). `onyx-core` only defines the contract.
#[async_trait]
pub trait Runtime: Send + Sync {
    /// The ready-to-run session this runtime produces.
    type Session: Session;

    /// Load a model from `resource`, described by `manifest`, into a session.
    async fn load(
        &self,
        resource: Resource,
        manifest: &ModelManifest,
    ) -> Result<Self::Session, Error>;
}

/// A loaded model, ready to execute forward passes.
#[async_trait]
pub trait Session: Send + Sync {
    /// The manifest the session was loaded from.
    fn manifest(&self) -> &ModelManifest;

    /// Declared input signature (name / dtype / shape).
    fn inputs(&self) -> &[IOSpec];

    /// Declared output signature (name / dtype / shape).
    fn outputs(&self) -> &[IOSpec];

    /// Run one forward pass over the named input tensors.
    async fn infer(&self, inputs: Inputs) -> Result<Outputs, Error>;
}

/// Declared I/O signature of a session: a named tensor slot with its dtype
/// and (possibly symbolic) shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IOSpec {
    pub name: String,
    pub dtype: DType,
    pub shape: Shape,
}
