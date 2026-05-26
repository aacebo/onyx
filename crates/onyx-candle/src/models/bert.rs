use std::sync::Arc;

use onyx_core::error::InferenceError;
use onyx_core::models::bert::*;
use onyx_core::models::*;
use onyx_core::resources::*;

pub struct BertModel {
    pub config: BertConfig,
    pub weights: Resource,
    pub vocab: Resource,
    pub tokenizer_config: BertTokenizerConfig,
}

impl Forward for BertModel {
    type Input = BertInput;
    type Output = BertOutput;

    fn forward<'a>(&'a self, _input: Self::Input) -> onyx_core::BoxFuture<'a, onyx_core::error::Result<Self::Output>> {
        Box::pin(async move { Err(InferenceError::Backend("BertModel::forward not yet implemented".into()).into()) })
    }
}

pub struct BertModelBuilder {
    resources: BertResourceConfig,
    reader: Arc<dyn io::Reader>,
    resolver: Arc<dyn net::Resolver>,
}

impl BertModelBuilder {
    pub fn new() -> Self {
        Self {
            resources: BertResourceConfig::default(),
            reader: Arc::new(io::StdReader::default()) as Arc<dyn io::Reader>,
            resolver: Arc::new(net::StdResolver::default()) as Arc<dyn net::Resolver>,
        }
    }

    pub fn resources(mut self, value: BertResourceConfig) -> Self {
        self.resources = value;
        self
    }

    pub fn reader(mut self, value: impl io::Reader + 'static) -> Self {
        self.reader = Arc::new(value) as Arc<dyn io::Reader>;
        self
    }

    pub fn resolver(mut self, value: impl net::Resolver + 'static) -> Self {
        self.resolver = Arc::new(value) as Arc<dyn net::Resolver>;
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
