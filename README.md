# `onyx-core` public API

```txt
onyx-core
  id
  error
  resource
  artifact
  manifest
  config
  tokenizer
  backend
  runtime
  pipeline
  task
  device
```

And your `lib.rs` should re-export the primary surface:

```rust
pub mod artifact;
pub mod backend;
pub mod config;
pub mod device;
pub mod error;
pub mod id;
pub mod manifest;
pub mod pipeline;
pub mod resource;
pub mod runtime;
pub mod task;
pub mod tokenizer;

pub mod prelude {
    pub use crate::artifact::*;
    pub use crate::backend::*;
    pub use crate::config::*;
    pub use crate::device::*;
    pub use crate::error::*;
    pub use crate::id::*;
    pub use crate::manifest::*;
    pub use crate::pipeline::*;
    pub use crate::resource::*;
    pub use crate::runtime::*;
    pub use crate::task::*;
    pub use crate::tokenizer::*;
}
```

---

# 1. IDs

```rust
// id.rs

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelId(String);

impl ModelId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for ModelId {
    type Err = crate::error::ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.trim().is_empty() {
            return Err(crate::error::ParseError::Empty);
        }

        Ok(Self(value.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Revision(String);

impl Revision {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

Keep `ModelId` intentionally simple. Do **not** over-parse HF syntax yet.

---

# 2. Errors

Use one public umbrella error plus smaller typed categories.

```rust
// error.rs

use std::fmt;

pub type Result<T> = std::result::Result<T, OnyxError>;

#[derive(Debug)]
pub enum OnyxError {
    Parse(ParseError),
    Resolve(ResolveError),
    Read(ReadError),
    Decode(DecodeError),
    Config(ConfigError),
    Tokenize(TokenizeError),
    Load(LoadError),
    Inference(InferenceError),
    Unsupported(UnsupportedError),
    Backend(Box<dyn std::error::Error + Send + Sync>),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, Clone)]
pub enum ParseError {
    Empty,
    InvalidModelId(String),
    InvalidUri(String),
}

#[derive(Debug)]
pub enum ResolveError {
    NotFound(String),
    Unavailable(String),
    UnsupportedScheme(String),
    PermissionDenied(String),
}

#[derive(Debug)]
pub enum ReadError {
    NotFound(String),
    Io(String),
}

#[derive(Debug)]
pub enum DecodeError {
    InvalidFormat(String),
    Json(String),
    Binary(String),
}

#[derive(Debug)]
pub enum ConfigError {
    MissingField(&'static str),
    InvalidField(&'static str),
    UnsupportedArchitecture(String),
    UnsupportedModelType(String),
}

#[derive(Debug)]
pub enum TokenizeError {
    InvalidInput(String),
    Backend(String),
}

#[derive(Debug)]
pub enum LoadError {
    MissingArtifact(String),
    InvalidWeights(String),
    InvalidConfig(String),
    Backend(String),
}

#[derive(Debug)]
pub enum InferenceError {
    InvalidInput(String),
    Backend(String),
}

#[derive(Debug)]
pub enum UnsupportedError {
    Task(crate::task::ModelTask),
    Architecture(crate::config::ModelArchitecture),
    Backend(String),
}
```

Do **not** make every integration expose raw errors as the public API. Backend crates can convert into `OnyxError`.

---

# 3. Resources

Resources are addressable things.

```rust
// resource.rs

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceUri(String);

impl ResourceUri {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceScheme {
    File,
    Http,
    Https,
    Hf,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct ResourceRequest {
    pub uri: ResourceUri,
}

#[derive(Debug, Clone)]
pub struct ResolvedResource {
    pub uri: ResourceUri,
    pub path: PathBuf,
}

pub trait ResourceResolver: Send + Sync {
    fn resolve<'a>(
        &'a self,
        request: ResourceRequest,
    ) -> crate::runtime::BoxFuture<'a, crate::error::Result<ResolvedResource>>;
}
```

Use boxed futures in core. Avoid `async-trait` unless you’re okay with that dependency.

---

# 4. Artifacts

Artifacts are model-relevant resources.

```rust
// artifact.rs

use crate::resource::{ResolvedResource, ResourceUri};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactKind {
    ModelConfig,
    Weights,
    WeightsIndex,
    Tokenizer,
    TokenizerConfig,
    SpecialTokensMap,
    GenerationConfig,
    PreprocessorConfig,
    Vocab,
    Merges,
    AddedTokens,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactFormat {
    Json,
    Safetensors,
    PytorchBin,
    Onnx,
    GGUF,
    Text,
    Binary,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ArtifactSpec {
    pub kind: ArtifactKind,
    pub uri: ResourceUri,
    pub format: ArtifactFormat,
    pub required: bool,
}

impl ArtifactSpec {
    pub fn required(
        kind: ArtifactKind,
        uri: impl Into<String>,
        format: ArtifactFormat,
    ) -> Self {
        Self {
            kind,
            uri: ResourceUri::new(uri),
            format,
            required: true,
        }
    }

    pub fn optional(
        kind: ArtifactKind,
        uri: impl Into<String>,
        format: ArtifactFormat,
    ) -> Self {
        Self {
            kind,
            uri: ResourceUri::new(uri),
            format,
            required: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedArtifact {
    pub kind: ArtifactKind,
    pub format: ArtifactFormat,
    pub resource: ResolvedResource,
}
```

---

# 5. Task taxonomy

```rust
// task.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelTask {
    Embeddings,
    TextGeneration,
    Text2TextGeneration,
    SequenceClassification,
    TokenClassification,
    QuestionAnswering,
    FillMask,
    Reranking,
    ImageClassification,
    ObjectDetection,
    SpeechRecognition,
}
```

---

# 6. Config taxonomy

```rust
// config/mod.rs

pub mod bert;
pub mod llama;
pub mod t5;

pub use bert::*;
pub use llama::*;
pub use t5::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelArchitecture {
    Bert,
    Roberta,
    DistilBert,
    Deberta,
    Gpt2,
    Llama,
    Mistral,
    T5,
    Bart,
    Clip,
    Whisper,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelFamily {
    EncoderOnly,
    DecoderOnly,
    EncoderDecoder,
    SentenceTransformer,
    Vision,
    Audio,
    Multimodal,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum NativeModelConfig {
    Bert(BertConfig),
    Llama(LlamaConfig),
    T5(T5Config),
    Unknown(serde_json::Value),
}

impl NativeModelConfig {
    pub fn architecture(&self) -> ModelArchitecture {
        match self {
            Self::Bert(_) => ModelArchitecture::Bert,
            Self::Llama(_) => ModelArchitecture::Llama,
            Self::T5(_) => ModelArchitecture::T5,
            Self::Unknown(_) => ModelArchitecture::Unknown,
        }
    }
}
```

### BERT config

```rust
// config/bert.rs

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BertConfig {
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub num_hidden_layers: usize,
    pub num_attention_heads: usize,
    pub intermediate_size: usize,

    #[serde(default)]
    pub hidden_act: Activation,

    #[serde(default = "default_dropout")]
    pub hidden_dropout_prob: f64,

    #[serde(default = "default_dropout")]
    pub attention_probs_dropout_prob: f64,

    pub max_position_embeddings: usize,

    #[serde(default = "default_type_vocab_size")]
    pub type_vocab_size: usize,

    #[serde(default = "default_initializer_range")]
    pub initializer_range: f64,

    #[serde(default = "default_layer_norm_eps")]
    pub layer_norm_eps: f64,

    #[serde(default)]
    pub pad_token_id: Option<u32>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Activation {
    Gelu,
    Relu,
    Silu,
    Tanh,
}

impl Default for Activation {
    fn default() -> Self {
        Self::Gelu
    }
}

fn default_dropout() -> f64 {
    0.1
}

fn default_type_vocab_size() -> usize {
    2
}

fn default_initializer_range() -> f64 {
    0.02
}

fn default_layer_norm_eps() -> f64 {
    1e-12
}
```

Only add configs as you support them. Don’t define 80 config structs up front.

---

# 7. Manifest

```rust
// manifest.rs

use crate::artifact::ArtifactSpec;
use crate::config::{ModelArchitecture, ModelFamily};
use crate::id::{ModelId, Revision};
use crate::task::ModelTask;

#[derive(Debug, Clone)]
pub struct ModelManifest {
    pub id: ModelId,
    pub revision: Option<Revision>,
    pub architecture: ModelArchitecture,
    pub family: ModelFamily,
    pub tasks: Vec<ModelTask>,
    pub artifacts: Vec<ArtifactSpec>,
    pub tokenizer: Option<TokenizerManifest>,
    pub processor: Option<ProcessorManifest>,
    pub backend_hints: BackendHints,
}

impl ModelManifest {
    pub fn artifact(&self, kind: crate::artifact::ArtifactKind) -> Option<&ArtifactSpec> {
        self.artifacts.iter().find(|artifact| artifact.kind == kind)
    }

    pub fn require_artifact(
        &self,
        kind: crate::artifact::ArtifactKind,
    ) -> crate::error::Result<&ArtifactSpec> {
        self.artifact(kind).ok_or_else(|| {
            crate::error::OnyxError::Load(crate::error::LoadError::MissingArtifact(format!(
                "{kind:?}"
            )))
        })
    }
}

#[derive(Debug, Clone)]
pub struct TokenizerManifest {
    pub kind: TokenizerKind,
    pub max_length: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenizerKind {
    Tokenizers,
    SentencePiece,
    Custom,
}

#[derive(Debug, Clone)]
pub struct ProcessorManifest {
    pub kind: ProcessorKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessorKind {
    Image,
    Audio,
    Multimodal,
    Custom,
}

#[derive(Debug, Clone, Default)]
pub struct BackendHints {
    pub preferred_backend: Option<String>,
    pub dtype: Option<crate::device::DType>,
    pub device: Option<crate::device::DevicePreference>,
}
```

---

# 8. Device and dtype

Keep this abstract. Do not use Candle device types.

```rust
// device.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceKind {
    Cpu,
    Cuda,
    Metal,
    Vulkan,
    WebGpu,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DevicePreference {
    Auto,
    Cpu,
    Gpu,
    Specific(DeviceKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DType {
    F32,
    F16,
    BF16,
    I64,
    I32,
    U32,
    U8,
}
```

---

# 9. Backend

Do not over-abstract tensors yet. Keep v0.1 backend metadata/factory oriented.

```rust
// backend.rs

use crate::device::{DType, DeviceKind};

#[derive(Debug, Clone)]
pub struct BackendCapabilities {
    pub devices: Vec<DeviceKind>,
    pub dtypes: Vec<DType>,
    pub supports_quantization: bool,
    pub supports_training: bool,
}

pub trait Backend: Send + Sync + 'static {
    fn name(&self) -> &'static str;

    fn capabilities(&self) -> BackendCapabilities;
}
```

For now, avoid:

```rust
trait Tensor
```

It will become a design tarpit unless you actually need backend-neutral tensor math.

---

# 10. Runtime

```rust
// runtime.rs

use std::future::Future;
use std::pin::Pin;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait Runtime: Send + Sync + 'static {
    fn spawn<'a>(
        &'a self,
        task: BoxFuture<'static, ()>,
    ) -> BoxFuture<'a, crate::error::Result<()>>;
}
```

Honestly, you may not need this immediately. If it adds friction, defer it.

For v0.1, the most useful runtime abstraction may just be `BoxFuture`.

---

# 11. Tokenizer

```rust
// tokenizer.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenId(pub u32);

#[derive(Debug, Clone)]
pub struct TokenizedInput {
    pub input_ids: Vec<TokenId>,
    pub attention_mask: Vec<u8>,
    pub token_type_ids: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct TokenizedBatch {
    pub input_ids: Vec<Vec<TokenId>>,
    pub attention_mask: Vec<Vec<u8>>,
    pub token_type_ids: Option<Vec<Vec<u8>>>,
}

#[derive(Debug, Clone)]
pub struct TokenizerOptions {
    pub padding: PaddingStrategy,
    pub truncation: TruncationStrategy,
    pub max_length: Option<usize>,
}

impl Default for TokenizerOptions {
    fn default() -> Self {
        Self {
            padding: PaddingStrategy::BatchLongest,
            truncation: TruncationStrategy::DoNotTruncate,
            max_length: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaddingStrategy {
    DoNotPad,
    BatchLongest,
    MaxLength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruncationStrategy {
    DoNotTruncate,
    LongestFirst,
    OnlyFirst,
    OnlySecond,
}

pub trait Tokenizer: Send + Sync {
    fn encode(
        &self,
        input: &str,
        options: &TokenizerOptions,
    ) -> crate::error::Result<TokenizedInput>;

    fn encode_batch(
        &self,
        input: &[String],
        options: &TokenizerOptions,
    ) -> crate::error::Result<TokenizedBatch>;

    fn decode(&self, tokens: &[TokenId]) -> crate::error::Result<String>;
}
```

Do not expose `tokenizers::Encoding`.

---

# 12. Pipeline API

This is the most important part.

```rust
// pipeline.rs

use crate::runtime::BoxFuture;

pub trait Pipeline: Send + Sync {
    type Input;
    type Output;

    fn run<'a>(
        &'a self,
        input: Self::Input,
    ) -> BoxFuture<'a, crate::error::Result<Self::Output>>;
}
```

## Embeddings

```rust
#[derive(Debug, Clone)]
pub struct TextBatch {
    pub texts: Vec<String>,
}

impl<T> From<T> for TextBatch
where
    T: IntoIterator,
    T::Item: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            texts: value.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddingOutput {
    pub embeddings: Vec<Vec<f32>>,
    pub dimensions: usize,
}

#[derive(Debug, Clone)]
pub struct EmbeddingOptions {
    pub normalize: bool,
    pub pooling: PoolingStrategy,
}

impl Default for EmbeddingOptions {
    fn default() -> Self {
        Self {
            normalize: true,
            pooling: PoolingStrategy::Mean,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolingStrategy {
    Cls,
    Mean,
    Max,
    LastToken,
}

pub trait EmbeddingPipeline: Send + Sync {
    fn embed<'a>(
        &'a self,
        input: TextBatch,
    ) -> BoxFuture<'a, crate::error::Result<EmbeddingOutput>>;
}
```

## Text generation

```rust
#[derive(Debug, Clone)]
pub struct GenerationInput {
    pub prompt: String,
    pub options: GenerationOptions,
}

#[derive(Debug, Clone)]
pub struct GenerationOptions {
    pub max_new_tokens: Option<usize>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<usize>,
    pub repetition_penalty: Option<f32>,
    pub stop: Vec<String>,
}

impl Default for GenerationOptions {
    fn default() -> Self {
        Self {
            max_new_tokens: Some(128),
            temperature: Some(0.8),
            top_p: None,
            top_k: None,
            repetition_penalty: None,
            stop: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenerationOutput {
    pub text: String,
    pub token_count: Option<usize>,
}

pub trait TextGenerationPipeline: Send + Sync {
    fn generate<'a>(
        &'a self,
        input: GenerationInput,
    ) -> BoxFuture<'a, crate::error::Result<GenerationOutput>>;
}
```

## Classification

```rust
#[derive(Debug, Clone)]
pub struct ClassificationInput {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct ClassificationOutput {
    pub labels: Vec<ScoredLabel>,
}

#[derive(Debug, Clone)]
pub struct ScoredLabel {
    pub label: String,
    pub score: f32,
}

pub trait SequenceClassificationPipeline: Send + Sync {
    fn classify<'a>(
        &'a self,
        input: ClassificationInput,
    ) -> BoxFuture<'a, crate::error::Result<ClassificationOutput>>;
}
```

---

# 13. Loading context

Core should define a generic loading context used by backend/pipeline crates.

```rust
// pipeline.rs or load.rs

use std::sync::Arc;

use crate::artifact::ResolvedArtifact;
use crate::backend::Backend;
use crate::config::NativeModelConfig;
use crate::manifest::ModelManifest;
use crate::resource::ResourceResolver;

pub struct LoadContext<B>
where
    B: Backend,
{
    pub backend: Arc<B>,
    pub resolver: Arc<dyn ResourceResolver>,
    pub manifest: ModelManifest,
    pub config: NativeModelConfig,
    pub artifacts: Vec<ResolvedArtifact>,
}
```

This lets `onyx-candle` consume a normalized context without owning the entire resolver/manifest world.

---

# 14. Registry API

For `Auto*` APIs, keep registries task-specific.

```rust
// registry.rs maybe later; could live in pipeline.rs initially

use std::collections::HashMap;
use std::sync::Arc;

use crate::backend::Backend;
use crate::config::ModelArchitecture;
use crate::runtime::BoxFuture;

pub struct EmbeddingFactory<B: Backend> {
    pub architecture: ModelArchitecture,
    pub load: for<'a> fn(
        crate::pipeline::LoadContext<B>,
    ) -> BoxFuture<'a, crate::error::Result<Box<dyn EmbeddingPipeline>>>,
}

pub struct EmbeddingRegistry<B: Backend> {
    factories: HashMap<ModelArchitecture, EmbeddingFactory<B>>,
}

impl<B: Backend> EmbeddingRegistry<B> {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register(mut self, factory: EmbeddingFactory<B>) -> Self {
        self.factories.insert(factory.architecture, factory);
        self
    }

    pub fn get(&self, architecture: ModelArchitecture) -> Option<&EmbeddingFactory<B>> {
        self.factories.get(&architecture)
    }
}
```

This is enough for:

```rust
AutoEmbeddings::builder()
    .backend(Candle::auto())
    .resolver(HfHubResolver::default())
    .registry(onyx_candle::embeddings::registry())
    .from_pretrained("sentence-transformers/all-MiniLM-L6-v2")
    .await?;
```

---

# What should **not** be in `onyx-core`

Do not include:

```rust
candle_core::Tensor
candle_core::Device
candle_core::DType
candle_nn::VarBuilder
hf_hub::api::tokio::Api
tokenizers::Tokenizer
tokio::runtime::Runtime
tokio::task::JoinHandle
```

Also do not include concrete model structs:

```rust
CandleBertModel
BertEmbeddingModel
LlamaGenerator
MiniLmEmbeddings
```

Those belong in integration/pipeline crates.

---

# Recommended initial public modules

For v0.1, I’d literally start with this:

```txt
src/
  lib.rs
  error.rs
  id.rs
  resource.rs
  artifact.rs
  manifest.rs
  task.rs
  device.rs
  backend.rs
  runtime.rs
  tokenizer.rs
  pipeline.rs
  config/
    mod.rs
    bert.rs
```

Defer these until needed:

```txt
registry.rs
processor.rs
quantization.rs
generation.rs
chat.rs
image.rs
audio.rs
```

---

# The minimal v0.1 public API contract

If you want the smallest credible core, ship only this:

```rust
pub use id::{ModelId, Revision};

pub use error::{OnyxError, Result};

pub use resource::{
    ResourceUri,
    ResourceRequest,
    ResolvedResource,
    ResourceResolver,
};

pub use artifact::{
    ArtifactKind,
    ArtifactFormat,
    ArtifactSpec,
    ResolvedArtifact,
};

pub use manifest::{
    ModelManifest,
    TokenizerManifest,
    BackendHints,
};

pub use config::{
    ModelArchitecture,
    ModelFamily,
    NativeModelConfig,
    BertConfig,
};

pub use task::ModelTask;

pub use device::{
    DeviceKind,
    DevicePreference,
    DType,
};

pub use backend::{
    Backend,
    BackendCapabilities,
};

pub use tokenizer::{
    Tokenizer,
    TokenId,
    TokenizedInput,
    TokenizedBatch,
    TokenizerOptions,
    PaddingStrategy,
    TruncationStrategy,
};

pub use pipeline::{
    Pipeline,
    EmbeddingPipeline,
    TextGenerationPipeline,
    TextBatch,
    EmbeddingOutput,
    EmbeddingOptions,
    PoolingStrategy,
    GenerationInput,
    GenerationOptions,
    GenerationOutput,
};
```

That is enough to build:

```txt
onyx-tokenizers
onyx-hf-hub
onyx-candle
onyx-pipeline-embeddings
```

without contaminating core.

---

# Blunt architectural advice

Do **not** make `onyx-core` the place where everything lives.

Make it the place where everything agrees.

A good `onyx-core` should feel boring, almost disappointingly so:

```txt
IDs
traits
DTOs
errors
manifests
config structs
task enums
```

That is exactly what makes the rest of the workspace maintainable.
