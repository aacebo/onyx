use crate::models::bert;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "model_type", rename_all = "snake_case")]
pub enum ModelConfig {
    Bert(bert::BertConfig),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ModelArchitecture;

    #[test]
    fn deserialize_bert_variant() {
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

        let cfg: ModelConfig = serde_json::from_str(json).unwrap();
        let ModelConfig::Bert(bert_cfg) = cfg;
        assert_eq!(bert_cfg.hidden_size, 768);
        assert_eq!(bert_cfg.architectures, vec![ModelArchitecture::BertForMaskedLM]);
    }

    #[test]
    fn serialize_writes_tag() {
        let cfg = ModelConfig::Bert(bert::BertConfig::default());
        let json = serde_json::to_string(&cfg).unwrap();
        assert!(json.contains("\"model_type\":\"bert\""), "got: {json}");
    }
}
