pub mod error;
pub mod io;
pub mod model;
pub mod resource;
pub mod runtime;
pub mod task;
pub mod tensor;
pub mod tokenizer;

pub use error::Error;
pub use io::{Inputs, Outputs, TensorMap};
pub use resource::Resource;
pub use runtime::{IOSpec, Runtime, Session};
pub use task::{Classifier, Embedder, TokenClassifier};
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
