use async_trait::async_trait;

use crate::{Annotation, Error};

/// Produces dense embeddings for input texts.
#[async_trait]
pub trait Embedder {
    /// Embed each input text, returning one vector per input.
    async fn embed(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, Error>;
}

/// Assigns one or more labeled scores to whole input sequences.
#[async_trait]
pub trait Classifier {
    /// Classify each input text, returning the annotations per input.
    async fn classify(&self, texts: &[&str]) -> Result<Vec<Vec<Annotation>>, Error>;
}

/// Assigns labeled scores to spans/tokens within each input sequence.
#[async_trait]
pub trait TokenClassifier {
    /// Classify the tokens of each input text, returning span annotations
    /// per input.
    async fn classify_tokens(&self, texts: &[&str]) -> Result<Vec<Vec<Annotation>>, Error>;
}
