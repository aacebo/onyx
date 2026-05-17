use async_trait::async_trait;

use crate::{Annotation, Tensor, error};

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelCapability {
    Embeddings,
    SequenceClassification,
    TokenClassification,
}

impl ModelCapability {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Embeddings => "embeddings",
            Self::SequenceClassification => "sequence_classification",
            Self::TokenClassification => "token_classification",
        }
    }
}

impl std::fmt::Display for ModelCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Produces dense embeddings for input texts.
#[async_trait]
pub trait Embedder {
    /// Embed each input text, returning one vector per input.
    async fn embed(&self, texts: &[&str]) -> Result<Vec<Tensor>, error::ModelError>;
}

/// Assigns one or more labeled scores to whole input sequences.
#[async_trait]
pub trait Classifier {
    /// Classify each input text, returning the annotations per input.
    async fn classify(&self, texts: &[&str]) -> Result<Vec<Vec<Annotation>>, error::ModelError>;
}

/// Assigns labeled scores to spans/tokens within each input sequence.
#[async_trait]
pub trait TokenClassifier {
    /// Classify the tokens of each input text, returning span annotations
    /// per input.
    async fn classify_tokens(
        &self,
        texts: &[&str],
    ) -> Result<Vec<Vec<Annotation>>, error::ModelError>;
}
