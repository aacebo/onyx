pub mod device;
pub mod error;
pub mod fs;
pub mod model;
pub mod net;
pub mod resource;
pub mod tensor;
pub mod token;

pub use error::{OnyxError, Result};
pub use resource::Resource;

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

pub trait Embedder {
    type Output;

    fn embed<'a>(&'a self, input: &'a [&'a str]) -> BoxFuture<'a, Vec<Self::Output>>;
}
