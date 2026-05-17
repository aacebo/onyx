#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelType {
    Bert,
    Roberta,
    DistilBert,
    Deberta,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelFeature {
    Embeddings,
    SequenceClassification,
    TokenClassification,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModelManifest {
    pub name: String,
    pub r#type: ModelType,
    pub features: Vec<ModelFeature>,
}
