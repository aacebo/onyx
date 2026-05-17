pub mod error;
pub mod model;
pub mod resource;
pub mod runtime;
pub mod task;
pub mod tensor;
pub mod tokenizer;

pub use error::Error;
pub use resource::Resource;
pub use runtime::Runtime;
pub use task::*;
pub use tensor::Tensor;
pub use tokenizer::Tokenizer;

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
