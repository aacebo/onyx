mod config;
mod resource;
mod tokenizer_config;
mod types;

pub use config::*;
pub use resource::*;
pub use tokenizer_config::*;
pub use types::*;

use crate::{Tensor, resources};

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

pub trait BertModel: Send + Sync {
    fn infer(&self, input: BertInput) -> impl Future<Output = crate::error::Result<BertOutput>> + Send;
}

pub trait AnyBertModel: Send + Sync {
    fn infer<'a>(&'a self, input: BertInput) -> crate::BoxFuture<'a, crate::error::Result<BertOutput>>;
}

impl<T: BertModel> AnyBertModel for T {
    fn infer<'a>(&'a self, input: BertInput) -> crate::BoxFuture<'a, crate::error::Result<BertOutput>> {
        Box::pin(async move { BertModel::infer(self, input).await })
    }
}
