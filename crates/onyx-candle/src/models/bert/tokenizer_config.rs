use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BertTokenizerConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_max_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_lower_case: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_basic_tokenize: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenize_chinese_chars: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip_accents: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_split: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_up_tokenization_spaces: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_special_tokens: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_side: Option<PaddingSide>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation_side: Option<TruncationSide>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pad_to_multiple_of: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pad_token_type_id: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_input_names: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_or_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unk_token: Option<StringOrAddedToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sep_token: Option<StringOrAddedToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pad_token: Option<StringOrAddedToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cls_token: Option<StringOrAddedToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_token: Option<StringOrAddedToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bos_token: Option<StringOrAddedToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eos_token: Option<StringOrAddedToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_tokens_decoder: Option<BTreeMap<String, AddedToken>>,
}

impl Default for BertTokenizerConfig {
    fn default() -> Self {
        Self {
            model_max_length: None,
            do_lower_case: None,
            do_basic_tokenize: None,
            tokenize_chinese_chars: None,
            strip_accents: None,
            never_split: None,
            clean_up_tokenization_spaces: None,
            split_special_tokens: None,
            padding_side: None,
            truncation_side: None,
            pad_to_multiple_of: None,
            pad_token_type_id: None,
            model_input_names: None,
            name_or_path: None,
            unk_token: None,
            sep_token: None,
            pad_token: None,
            cls_token: None,
            mask_token: None,
            bos_token: None,
            eos_token: None,
            added_tokens_decoder: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum StringOrAddedToken {
    String(String),
    Object(AddedToken),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddedToken {
    pub content: String,

    #[serde(default)]
    pub lstrip: bool,

    #[serde(default)]
    pub rstrip: bool,

    #[serde(default = "default_true")]
    pub normalized: bool,

    #[serde(default)]
    pub single_word: bool,

    #[serde(default)]
    pub special: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaddingSide {
    Left,
    Right,
}

impl PaddingSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

impl std::fmt::Display for PaddingSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TruncationSide {
    Left,
    Right,
}

impl TruncationSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

impl std::fmt::Display for TruncationSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_minimal_bert_base_uncased() {
        let json = r#"{"do_lower_case": true}"#;
        let cfg: BertTokenizerConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.do_lower_case, Some(true));
        assert_eq!(cfg.model_max_length, None);
    }

    #[test]
    fn deserialize_full_slow_tokenizer() {
        let json = r#"{
            "do_lower_case": true,
            "unk_token": "[UNK]",
            "sep_token": "[SEP]",
            "pad_token": "[PAD]",
            "cls_token": "[CLS]",
            "mask_token": "[MASK]",
            "tokenize_chinese_chars": true,
            "strip_accents": null,
            "model_max_length": 512,
            "name_or_path": "sentence-transformers/all-MiniLM-L6-v2"
        }"#;
        let cfg: BertTokenizerConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.do_lower_case, Some(true));
        assert_eq!(cfg.unk_token, Some(StringOrAddedToken::String("[UNK]".into())));
        assert_eq!(cfg.model_max_length, Some(512));
        assert_eq!(cfg.strip_accents, None);
        assert_eq!(cfg.name_or_path.as_deref(), Some("sentence-transformers/all-MiniLM-L6-v2"));
    }

    #[test]
    fn deserialize_mask_as_added_token_object() {
        let json = r#"{
            "mask_token": {
                "content": "[MASK]",
                "lstrip": true,
                "rstrip": false,
                "normalized": false,
                "single_word": false,
                "special": true
            },
            "model_input_names": ["input_ids", "attention_mask"]
        }"#;
        let cfg: BertTokenizerConfig = serde_json::from_str(json).unwrap();
        match cfg.mask_token {
            Some(StringOrAddedToken::Object(ref tok)) => {
                assert_eq!(tok.content, "[MASK]");
                assert!(tok.lstrip);
                assert!(!tok.normalized);
                assert!(tok.special);
            }
            other => panic!("expected AddedToken object, got {other:?}"),
        }
        assert_eq!(
            cfg.model_input_names.as_deref(),
            Some(&["input_ids".to_string(), "attention_mask".to_string()][..])
        );
    }

    #[test]
    fn deserialize_added_tokens_decoder() {
        let json = r#"{
            "added_tokens_decoder": {
                "0": {"content": "[PAD]", "lstrip": false, "rstrip": false, "normalized": false, "single_word": false, "special": true},
                "100": {"content": "[UNK]", "lstrip": false, "rstrip": false, "normalized": false, "single_word": false, "special": true}
            }
        }"#;
        let cfg: BertTokenizerConfig = serde_json::from_str(json).unwrap();
        let map = cfg.added_tokens_decoder.expect("decoder present");
        assert_eq!(map.get("0").map(|t| t.content.as_str()), Some("[PAD]"));
        assert_eq!(map.get("100").map(|t| t.content.as_str()), Some("[UNK]"));
    }

    #[test]
    fn deserialize_with_sides_and_pad_multiple() {
        let json = r#"{
            "padding_side": "right",
            "truncation_side": "right",
            "pad_to_multiple_of": 8,
            "pad_token_type_id": 0,
            "model_max_length": 2147483648
        }"#;
        let cfg: BertTokenizerConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.padding_side, Some(PaddingSide::Right));
        assert_eq!(cfg.truncation_side, Some(TruncationSide::Right));
        assert_eq!(cfg.pad_to_multiple_of, Some(8));
        assert_eq!(cfg.model_max_length, Some(2_147_483_648));
    }

    #[test]
    fn added_token_default_normalized_is_true() {
        let json = r#"{"content": "foo"}"#;
        let tok: AddedToken = serde_json::from_str(json).unwrap();
        assert!(tok.normalized);
        assert!(!tok.lstrip);
        assert!(!tok.special);
    }

    #[test]
    fn serde_roundtrip_preserves_set_fields() {
        let cfg = BertTokenizerConfig {
            do_lower_case: Some(true),
            model_max_length: Some(512),
            unk_token: Some(StringOrAddedToken::String("[UNK]".into())),
            ..Default::default()
        };
        let json = serde_json::to_string(&cfg).unwrap();
        let back: BertTokenizerConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back, cfg);
    }
}
