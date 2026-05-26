pub mod error;
pub mod models;
pub mod resources;
pub mod tensor;
pub mod tokens;

pub use error::{OnyxError, Result};
pub use resources::Resource;
pub use tensor::Tensor;

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

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
