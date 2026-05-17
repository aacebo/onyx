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
