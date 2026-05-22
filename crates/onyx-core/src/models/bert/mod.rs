mod config;
mod resource;

pub use config::*;
pub use resource::*;

use crate::Tensor;

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
