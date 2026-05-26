mod config;
mod resource;
mod tokenizer_config;
mod types;

use std::sync::Arc;

pub use config::*;
pub use resource::*;
pub use tokenizer_config::*;
pub use types::*;

use crate::{Error, Resource, Tensor, resources};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BertInput {
    pub input_ids: Tensor,
    pub attention_mask: Option<Tensor>,
    pub token_type_ids: Option<Tensor>,
    pub position_ids: Option<Tensor>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BertOutput {
    pub last_hidden_state: Tensor,
    pub pooled_output: Option<Tensor>,
    pub hidden_states: Option<Vec<Tensor>>,
    pub attentions: Option<Vec<Tensor>>,
}

pub struct BertModel {
    pub config: BertConfig,
    pub weights: Resource,
    pub vocab: Resource,
    pub tokenizer_config: BertTokenizerConfig,
}

impl crate::models::Forward for BertModel {
    type Input = BertInput;
    type Output = BertOutput;
    type Error = Error;

    async fn forward(&self, _input: Self::Input) -> Result<Self::Output, Self::Error> {
        Err(Error::message("BertModel::forward not yet implemented"))
    }
}

pub struct BertModelBuilder {
    resources: BertResourceConfig,
    reader: Arc<dyn resources::io::internal::AnyReader>,
    resolver: Arc<dyn resources::net::internal::AnyResolver>,
}

impl BertModelBuilder {
    pub fn new() -> Self {
        Self {
            resources: BertResourceConfig::default(),
            reader: Arc::new(resources::io::StdReader::default()),
            resolver: Arc::new(resources::net::StdResolver::default()),
        }
    }

    pub fn resources(mut self, value: BertResourceConfig) -> Self {
        self.resources = value;
        self
    }

    pub fn reader(mut self, value: impl resources::io::Reader + 'static) -> Self {
        self.reader = Arc::new(value);
        self
    }

    pub fn resolver(mut self, value: impl resources::net::Resolver + 'static) -> Self {
        self.resolver = Arc::new(value);
        self
    }

    pub async fn build(self) -> Result<BertModel, Error> {
        let config = match self.resources.config {
            UriOrConfig::Config(v) => v,
            UriOrConfig::Uri(uri) => {
                let resource = self.resolver.resolve(&uri).await.map_err(Error::source)?;
                let bytes = self.reader.read(&resource).await.map_err(Error::source)?;
                resource.format.decode(&bytes)?
            }
        };

        let tokenizer_config = match self.resources.tokenizer_config {
            UriOrTokenizerConfig::Config(v) => v,
            UriOrTokenizerConfig::Uri(uri) => {
                let resource = self.resolver.resolve(&uri).await.map_err(Error::source)?;
                let bytes = self.reader.read(&resource).await.map_err(Error::source)?;
                resource.format.decode(&bytes)?
            }
        };

        Ok(BertModel {
            config,
            weights: self.resolver.resolve(&self.resources.weights).await.map_err(Error::source)?,
            vocab: self.resolver.resolve(&self.resources.vocab).await.map_err(Error::source)?,
            tokenizer_config,
        })
    }
}
