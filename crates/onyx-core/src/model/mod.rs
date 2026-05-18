mod architecture;
mod capability;

pub use architecture::*;
pub use capability::*;

use async_trait::async_trait;

use crate::{error, pipeline, tensor};

/// A loaded model: one pipeline per capability the manifest declares.
#[derive(Default)]
pub struct Model {
    embedder: Option<Box<dyn pipeline::Embedder>>,
    classifier: Option<Box<dyn pipeline::Classifier>>,
    token_classifier: Option<Box<dyn pipeline::TokenClassifier>>,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn embedder(&self) -> Option<&dyn pipeline::Embedder> {
        self.embedder.as_deref()
    }

    pub fn classifier(&self) -> Option<&dyn pipeline::Classifier> {
        self.classifier.as_deref()
    }

    pub fn token_classifier(&self) -> Option<&dyn pipeline::TokenClassifier> {
        self.token_classifier.as_deref()
    }
}

#[async_trait]
pub trait ModelRegistry {
    async fn exists(&self, id: &ModelId) -> bool;
    async fn get(&self, id: &ModelId) -> Result<Option<Model>, error::ModelError>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelManifest {
    pub id: ModelId,
    pub architecture: ModelArchitecture,
    pub capabilities: Vec<ModelCapability>,
    pub inputs: tensor::TensorSchema,
    pub outputs: tensor::TensorSchema,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ModelId {
    group: Box<str>,
    name: Box<str>,
}

impl std::str::FromStr for ModelId {
    type Err = error::ModelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (group, name) = match s.split_once("/") {
            None => return Err(error::ModelError::parse("invalid model id format")),
            Some(v) => v,
        };

        Ok(Self {
            group: group.into(),
            name: name.into(),
        })
    }
}

impl std::fmt::Debug for ModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for ModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", &self.group, &self.name)
    }
}

impl serde::Serialize for ModelId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ModelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::str::FromStr;

        let value = String::deserialize(deserializer)?;
        Self::from_str(&value).map_err(serde::de::Error::custom)
    }
}
