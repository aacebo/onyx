pub mod error;
pub(crate) mod internal;
pub mod models;
pub mod pipelines;
pub mod resources;
pub mod tensor;
pub mod tokens;

pub use error::Error;
pub use models::Model;
pub use pipelines::Pipeline;
pub use resources::Resource;
pub use tensor::Tensor;

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Annotation {
    pub label: String,
    pub score: f32,
    pub span: Span,
    pub text: String,
}
