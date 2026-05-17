use async_trait::async_trait;

use crate::{Error, TensorMap, tensor::TensorSpec};

/// A loaded model, ready to execute forward passes.
#[async_trait]
pub trait ModelSession: Send + Sync {
    /// The manifest the session was loaded from.
    fn manifest(&self) -> &ModelManifest;

    /// Run one forward pass over the named input tensors.
    async fn infer(&self, inputs: TensorMap) -> Result<TensorMap, Error>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelType {
    #[serde(rename = "BERT")]
    Bert,

    #[serde(rename = "RoBERTa")]
    Roberta,

    #[serde(rename = "DistilBERT")]
    DistilBert,

    #[serde(rename = "DeBERTa")]
    Deberta,
}

impl ModelType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bert => "BERT",
            Self::Roberta => "RoBERTa",
            Self::DistilBert => "DistilBERT",
            Self::Deberta => "DeBERTa",
        }
    }
}

impl std::fmt::Display for ModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelFeature {
    Embeddings,
    SequenceClassification,
    TokenClassification,
}

impl ModelFeature {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Embeddings => "embeddings",
            Self::SequenceClassification => "sequence-classification",
            Self::TokenClassification => "token-classification",
        }
    }
}

impl std::fmt::Display for ModelFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModelManifest {
    pub name: String,
    pub r#type: ModelType,
    pub features: Vec<ModelFeature>,
    pub inputs: Vec<TensorSpec>,
    pub outputs: Vec<TensorSpec>,
}
