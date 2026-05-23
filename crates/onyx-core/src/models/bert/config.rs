use crate::models::{HiddenAct, ModelArchitecture, PositionEmbeddingType};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BertConfig {
    pub architectures: Vec<ModelArchitecture>,
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub num_hidden_layers: usize,
    pub num_attention_heads: usize,
    pub intermediate_size: usize,
    pub hidden_act: HiddenAct,
    pub hidden_dropout_prob: f32,
    pub attention_probs_dropout_prob: f32,
    pub max_position_embeddings: usize,
    pub type_vocab_size: usize,
    pub initializer_range: f32,
    pub layer_norm_eps: f64,
    pub pad_token_id: usize,
    pub position_embedding_type: PositionEmbeddingType,
}

impl Default for BertConfig {
    fn default() -> Self {
        Self {
            architectures: Vec::new(),
            vocab_size: 30522,
            hidden_size: 768,
            num_hidden_layers: 12,
            num_attention_heads: 12,
            intermediate_size: 3072,
            hidden_act: HiddenAct::Gelu,
            hidden_dropout_prob: 0.1,
            attention_probs_dropout_prob: 0.1,
            max_position_embeddings: 512,
            type_vocab_size: 2,
            initializer_range: 0.02,
            layer_norm_eps: 1e-12,
            pad_token_id: 0,
            position_embedding_type: PositionEmbeddingType::Absolute,
        }
    }
}

impl BertConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_architectures(mut self, architectures: Vec<ModelArchitecture>) -> Self {
        self.architectures = architectures;
        self
    }

    pub fn with_vocab_size(mut self, vocab_size: usize) -> Self {
        self.vocab_size = vocab_size;
        self
    }

    pub fn with_hidden_size(mut self, hidden_size: usize) -> Self {
        self.hidden_size = hidden_size;
        self
    }

    pub fn with_num_hidden_layers(mut self, num_hidden_layers: usize) -> Self {
        self.num_hidden_layers = num_hidden_layers;
        self
    }

    pub fn with_num_attention_heads(mut self, num_attention_heads: usize) -> Self {
        self.num_attention_heads = num_attention_heads;
        self
    }

    pub fn with_intermediate_size(mut self, intermediate_size: usize) -> Self {
        self.intermediate_size = intermediate_size;
        self
    }

    pub fn with_hidden_act(mut self, hidden_act: HiddenAct) -> Self {
        self.hidden_act = hidden_act;
        self
    }

    pub fn with_hidden_dropout_prob(mut self, hidden_dropout_prob: f32) -> Self {
        self.hidden_dropout_prob = hidden_dropout_prob;
        self
    }

    pub fn with_attention_probs_dropout_prob(mut self, attention_probs_dropout_prob: f32) -> Self {
        self.attention_probs_dropout_prob = attention_probs_dropout_prob;
        self
    }

    pub fn with_max_position_embeddings(mut self, max_position_embeddings: usize) -> Self {
        self.max_position_embeddings = max_position_embeddings;
        self
    }

    pub fn with_type_vocab_size(mut self, type_vocab_size: usize) -> Self {
        self.type_vocab_size = type_vocab_size;
        self
    }

    pub fn with_initializer_range(mut self, initializer_range: f32) -> Self {
        self.initializer_range = initializer_range;
        self
    }

    pub fn with_layer_norm_eps(mut self, layer_norm_eps: f64) -> Self {
        self.layer_norm_eps = layer_norm_eps;
        self
    }

    pub fn with_pad_token_id(mut self, pad_token_id: usize) -> Self {
        self.pad_token_id = pad_token_id;
        self
    }

    pub fn with_position_embedding_type(mut self, position_embedding_type: PositionEmbeddingType) -> Self {
        self.position_embedding_type = position_embedding_type;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let config = BertConfig::default();
        assert_eq!(config.vocab_size, 30522);
        assert_eq!(config.hidden_size, 768);
        assert_eq!(config.num_hidden_layers, 12);
        assert_eq!(config.hidden_act, HiddenAct::Gelu);
        assert_eq!(config.position_embedding_type, PositionEmbeddingType::Absolute);
    }

    #[test]
    fn builder_chains() {
        let config = BertConfig::default()
            .with_hidden_size(1024)
            .with_num_hidden_layers(24)
            .with_architectures(vec![ModelArchitecture::BertForSequenceClassification]);

        assert_eq!(config.hidden_size, 1024);
        assert_eq!(config.num_hidden_layers, 24);
        assert_eq!(config.architectures, vec![ModelArchitecture::BertForSequenceClassification]);
        // untouched fields keep their defaults
        assert_eq!(config.vocab_size, 30522);
        assert_eq!(config.num_attention_heads, 12);
    }

    #[test]
    fn serde_roundtrip() {
        let config = BertConfig::default()
            .with_hidden_size(1024)
            .with_hidden_act(HiddenAct::GeluNew);

        let json = serde_json::to_string(&config).unwrap();
        let back: BertConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back, config);
    }

    #[test]
    fn deserialize_partial() {
        let config: BertConfig = serde_json::from_str(r#"{"hidden_size": 1024}"#).unwrap();
        assert_eq!(config.hidden_size, 1024);
        // missing keys fall back to defaults
        assert_eq!(config.vocab_size, 30522);
        assert_eq!(config.num_hidden_layers, 12);
    }

    #[test]
    fn deserialize_config_sample() {
        let json = r#"{
            "model_type": "bert",
            "architectures": ["BertForMaskedLM"],
            "vocab_size": 30522,
            "hidden_size": 768,
            "num_hidden_layers": 12,
            "num_attention_heads": 12,
            "intermediate_size": 3072,
            "hidden_act": "gelu",
            "hidden_dropout_prob": 0.1,
            "attention_probs_dropout_prob": 0.1,
            "max_position_embeddings": 512,
            "type_vocab_size": 2,
            "initializer_range": 0.02,
            "layer_norm_eps": 1e-12,
            "pad_token_id": 0,
            "position_embedding_type": "absolute"
        }"#;

        let config: BertConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.hidden_size, 768);
        assert_eq!(config.num_attention_heads, 12);
        assert_eq!(config.hidden_act, HiddenAct::Gelu);
        assert_eq!(config.architectures, vec![ModelArchitecture::BertForMaskedLM]);
    }
}
