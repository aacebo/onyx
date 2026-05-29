mod config;
mod resource;
mod types;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert as ct_bert;
pub use config::*;
use onyx_core::BoxFuture;
use onyx_core::error::{InferenceError, LoadError, TokenizeError};
use onyx_core::model::Forward;
use onyx_core::resource::*;
pub use resource::*;
pub use types::*;

#[derive(Debug, Clone)]
pub struct BertInput {
    pub input_ids: Tensor,
    pub token_type_ids: Tensor,
    pub attention_mask: Option<Tensor>,
}

#[derive(Debug, Clone)]
pub struct BertOutput {
    pub last_hidden_state: Tensor,
}

pub struct BertModel {
    inner: ct_bert::BertModel,
    tokenizer: tokenizers::Tokenizer,
    device: Device,
}

impl BertModel {
    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn tokenizer(&self) -> &tokenizers::Tokenizer {
        &self.tokenizer
    }

    /// Tokenize a single string and run a forward pass, returning the last hidden state.
    pub fn encode(&self, text: &str) -> onyx_core::error::Result<Tensor> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| TokenizeError::Backend(e.to_string()))?;

        let ids = encoding.get_ids();
        let type_ids = encoding.get_type_ids();
        let mask = encoding.get_attention_mask();
        let seq_len = ids.len();
        let input_ids =
            Tensor::from_slice(ids, (1, seq_len), &self.device).map_err(|e| InferenceError::Backend(e.to_string()))?;
        let token_type_ids =
            Tensor::from_slice(type_ids, (1, seq_len), &self.device).map_err(|e| InferenceError::Backend(e.to_string()))?;
        let attention_mask =
            Tensor::from_slice(mask, (1, seq_len), &self.device).map_err(|e| InferenceError::Backend(e.to_string()))?;

        self.inner
            .forward(&input_ids, &token_type_ids, Some(&attention_mask))
            .map_err(|e| InferenceError::Backend(e.to_string()).into())
    }
}

impl Forward for BertModel {
    type Input = BertInput;
    type Output = BertOutput;

    fn forward<'a>(&'a self, input: Self::Input) -> BoxFuture<'a, onyx_core::error::Result<Self::Output>> {
        Box::pin(async move {
            let last_hidden_state = self
                .inner
                .forward(&input.input_ids, &input.token_type_ids, input.attention_mask.as_ref())
                .map_err(|e| InferenceError::Backend(e.to_string()))?;
            Ok(BertOutput { last_hidden_state })
        })
    }
}

pub struct BertModelBuilder {
    resources: BertResourceConfig,
    reader: Option<std::sync::Arc<dyn Reader>>,
    resolver: Option<std::sync::Arc<dyn Resolver>>,
    device: Device,
    dtype: DType,
}

impl BertModelBuilder {
    pub fn new() -> Self {
        Self {
            resources: BertResourceConfig::default(),
            reader: None,
            resolver: None,
            device: Device::Cpu,
            dtype: ct_bert::DTYPE,
        }
    }

    pub fn resources(mut self, value: BertResourceConfig) -> Self {
        self.resources = value;
        self
    }

    pub fn reader(mut self, value: impl Reader + 'static) -> Self {
        self.reader = Some(std::sync::Arc::new(value) as std::sync::Arc<dyn Reader>);
        self
    }

    pub fn resolver(mut self, value: impl Resolver + 'static) -> Self {
        self.resolver = Some(std::sync::Arc::new(value) as std::sync::Arc<dyn Resolver>);
        self
    }

    pub fn device(mut self, device: Device) -> Self {
        self.device = device;
        self
    }

    pub fn dtype(mut self, dtype: DType) -> Self {
        self.dtype = dtype;
        self
    }

    pub async fn build(self) -> onyx_core::error::Result<BertModel> {
        let reader = self.reader.expect("resource reader");
        let resolver = self.resolver.expect("resource resolver");
        let config: BertConfig = match self.resources.config {
            UriOrConfig::Config(v) => v,
            UriOrConfig::Uri(uri) => {
                let resource = resolver.resolve(&uri).await?;
                let bytes = reader.read(&resource).await?;
                serde_json::from_slice(&bytes).map_err(|err| onyx_core::error::DecodeError::Json(err.to_string()))?
            }
        };

        let tokenizer_uri = self.resources.tokenizer;
        let tokenizer_resource = resolver.resolve(&tokenizer_uri).await?;
        let tokenizer_bytes = reader.read(&tokenizer_resource).await?;
        let tokenizer = tokenizers::Tokenizer::from_bytes(&tokenizer_bytes).map_err(|e| TokenizeError::Backend(e.to_string()))?;
        let weights_resource = resolver.resolve(&self.resources.weights).await?;
        let weights_bytes = reader.read(&weights_resource).await?;
        let candle_config: ct_bert::Config = config.try_into()?;
        let vb = VarBuilder::from_buffered_safetensors(weights_bytes, self.dtype, &self.device)
            .map_err(|e| LoadError::InvalidWeights(e.to_string()))?;
        let inner = ct_bert::BertModel::load(vb, &candle_config).map_err(|e| LoadError::Backend(e.to_string()))?;

        Ok(BertModel {
            inner,
            tokenizer,
            device: self.device,
        })
    }
}

impl Default for BertModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}
