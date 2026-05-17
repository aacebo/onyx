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
