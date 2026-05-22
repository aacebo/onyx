mod config;
mod resource;

use std::sync::Arc;

pub use config::*;
pub use resource::*;

use crate::{Error, Tensor, resources};

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
}

impl BertModel {
    pub fn from_config(config: impl Into<BertConfig>) -> Self {
        Self { config: config.into() }
    }
}

impl From<BertConfig> for BertModel {
    fn from(value: BertConfig) -> Self {
        Self::from_config(value)
    }
}

pub struct BertModelBuilder {
    resources: BertResourceGroup,
    reader: Arc<dyn resources::io::internal::AnyReader>,
    resolver: Arc<dyn resources::net::internal::AnyResolver>,
}

impl BertModelBuilder {
    pub fn new() -> Self {
        Self {
            resources: BertResourceGroup::default(),
            reader: Arc::new(resources::io::StdReader::default()),
            resolver: Arc::new(resources::net::StdResolver::default()),
        }
    }

    pub fn resources(mut self, value: BertResourceGroup) -> Self {
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
        let config_resource = self.resolver.resolve(&self.resources.config).await.map_err(Error::source)?;
        let config_bytes = self.reader.read(&config_resource).await.map_err(Error::source)?;
        let config: BertConfig = config_resource.format.decode(&config_bytes)?;
        Ok(BertModel::from_config(config))
    }
}
