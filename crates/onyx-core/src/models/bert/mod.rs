mod config;
mod resource;
mod types;

use std::sync::Arc;

pub use config::*;
pub use resource::*;
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
    #[allow(unused)]
    config: BertConfig,

    #[allow(unused)]
    weights: Resource,

    #[allow(unused)]
    vocab: Resource,

    #[allow(unused)]
    tokenizer_config: Resource,
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

        Ok(BertModel {
            config,
            weights: self.resolver.resolve(&self.resources.weights).await.map_err(Error::source)?,
            vocab: self.resolver.resolve(&self.resources.vocab).await.map_err(Error::source)?,
            tokenizer_config: self
                .resolver
                .resolve(&self.resources.tokenizer_config)
                .await
                .map_err(Error::source)?,
        })
    }
}
