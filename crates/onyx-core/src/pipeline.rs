use async_trait::async_trait;

use crate::{Annotation, Tensor, error};

/// A composed inference flow (tokenize -> session -> decode) that satisfies
/// exactly one capability. Concrete pipelines additionally implement the
/// matching capability trait (Embedder / Classifier / TokenClassifier).
pub trait Pipeline: Send + Sync {}

/// Produces dense embeddings for input texts.
#[async_trait]
pub trait Embedder: Pipeline {
    /// Embed each input text, returning one vector per input.
    async fn embed(&self, texts: &[&str]) -> Result<Vec<Tensor>, error::ModelError>;
}

/// Assigns one or more labeled scores to whole input sequences.
#[async_trait]
pub trait Classifier: Pipeline {
    /// Classify each input text, returning the annotations per input.
    async fn classify(&self, texts: &[&str]) -> Result<Vec<Vec<Annotation>>, error::ModelError>;
}

/// Assigns labeled scores to spans/tokens within each input sequence.
#[async_trait]
pub trait TokenClassifier: Pipeline {
    /// Classify the tokens of each input text, returning span annotations
    /// per input.
    async fn classify_tokens(
        &self,
        texts: &[&str],
    ) -> Result<Vec<Vec<Annotation>>, error::ModelError>;
}
