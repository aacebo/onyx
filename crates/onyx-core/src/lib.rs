pub mod error;
#[cfg(feature = "huggingface")]
pub mod huggingface;
pub mod model;
pub mod pipeline;
pub mod resource;
pub mod tensor;
pub mod tokenizer;

pub use error::Error;
pub use model::Model;
pub use pipeline::Pipeline;
pub use tensor::Tensor;
pub use tokenizer::{Encoding, Tokenizer};

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
