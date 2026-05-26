use crate::models::bert;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "tokenizer_class")]
pub enum TokenizerConfig {
    #[serde(alias = "BertTokenizerFast")]
    BertTokenizer(bert::BertTokenizerConfig),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::bert::{PaddingSide, StringOrAddedToken};

    #[test]
    fn deserialize_bert_tokenizer() {
        let json = r#"{
            "tokenizer_class": "BertTokenizer",
            "do_lower_case": true,
            "unk_token": "[UNK]",
            "model_max_length": 512
        }"#;
        let cfg: TokenizerConfig = serde_json::from_str(json).unwrap();
        let TokenizerConfig::BertTokenizer(inner) = cfg;
        assert_eq!(inner.do_lower_case, Some(true));
        assert_eq!(inner.unk_token, Some(StringOrAddedToken::String("[UNK]".into())));
        assert_eq!(inner.model_max_length, Some(512));
    }

    #[test]
    fn deserialize_bert_tokenizer_fast_alias() {
        let json = r#"{
            "tokenizer_class": "BertTokenizerFast",
            "do_lower_case": false,
            "padding_side": "right"
        }"#;
        let cfg: TokenizerConfig = serde_json::from_str(json).unwrap();
        let TokenizerConfig::BertTokenizer(inner) = cfg;
        assert_eq!(inner.do_lower_case, Some(false));
        assert_eq!(inner.padding_side, Some(PaddingSide::Right));
    }

    #[test]
    fn serialize_writes_tag() {
        let cfg = TokenizerConfig::BertTokenizer(bert::BertTokenizerConfig::default());
        let json = serde_json::to_string(&cfg).unwrap();
        assert!(json.contains("\"tokenizer_class\":\"BertTokenizer\""), "got: {json}");
    }
}
