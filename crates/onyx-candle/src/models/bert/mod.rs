mod config;
mod resource;
mod tokenizer_config;
mod types;

pub use config::*;
use onyx_core::model::Forward;
use onyx_core::resource::*;
use onyx_core::{BoxFuture, Tensor};
pub use resource::*;
pub use tokenizer_config::*;
pub use types::*;

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

impl Forward for BertModel {
    type Input = BertInput;
    type Output = BertOutput;

    fn forward<'a>(&'a self, _input: Self::Input) -> BoxFuture<'a, onyx_core::error::Result<Self::Output>> {
        Box::pin(
            async move { Err(onyx_core::error::InferenceError::Backend("BertModel::forward not yet implemented".into()).into()) },
        )
    }
}

pub struct BertModelBuilder {
    resources: BertResourceConfig,
    reader: std::sync::Arc<dyn io::Reader>,
    resolver: std::sync::Arc<dyn net::Resolver>,
}

impl BertModelBuilder {
    pub fn new() -> Self {
        Self {
            resources: BertResourceConfig::default(),
            reader: std::sync::Arc::new(io::StdReader::default()) as std::sync::Arc<dyn io::Reader>,
            resolver: std::sync::Arc::new(net::StdResolver::default()) as std::sync::Arc<dyn net::Resolver>,
        }
    }

    pub fn resources(mut self, value: BertResourceConfig) -> Self {
        self.resources = value;
        self
    }

    pub fn reader(mut self, value: impl io::Reader + 'static) -> Self {
        self.reader = std::sync::Arc::new(value) as std::sync::Arc<dyn io::Reader>;
        self
    }

    pub fn resolver(mut self, value: impl net::Resolver + 'static) -> Self {
        self.resolver = std::sync::Arc::new(value) as std::sync::Arc<dyn net::Resolver>;
        self
    }

    pub async fn build(self) -> onyx_core::error::Result<BertModel> {
        let config = match self.resources.config {
            UriOrConfig::Config(v) => v,
            UriOrConfig::Uri(uri) => {
                let resource = self.resolver.resolve(&uri).await?;
                let bytes = self.reader.read(&resource).await?;
                resource.format.decode(&bytes)?
            }
        };

        let tokenizer_config = match self.resources.tokenizer_config {
            UriOrTokenizerConfig::Config(v) => v,
            UriOrTokenizerConfig::Uri(uri) => {
                let resource = self.resolver.resolve(&uri).await?;
                let bytes = self.reader.read(&resource).await?;
                resource.format.decode(&bytes)?
            }
        };

        Ok(BertModel {
            config,
            weights: self.resolver.resolve(&self.resources.weights).await?,
            vocab: self.resolver.resolve(&self.resources.vocab).await?,
            tokenizer_config,
        })
    }
}

impl Default for BertModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}
