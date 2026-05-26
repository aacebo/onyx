mod config;
mod resource;
mod tokenizer_config;
mod types;

pub use config::*;
pub use resource::*;
pub use tokenizer_config::*;
pub use types::*;

use crate::{BoxFuture, Tensor, resources};

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
    fn infer(&self, input: BertInput) -> BoxFuture<'_, crate::error::Result<BertOutput>>;
}
