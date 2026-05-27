use onyx_core::resource;

use super::*;

pub struct BertResourceConfig {
    pub config: UriOrConfig,
    pub weights: resource::Uri,
    pub tokenizer: resource::Uri,
}

impl Default for BertResourceConfig {
    fn default() -> Self {
        Self::mini_lm_l6_v2()
    }
}

fn hf(model: &str, file: &str) -> resource::Uri {
    format!("https://huggingface.co/{model}/resolve/main/{file}").parse().unwrap()
}

fn preset(model: &str) -> BertResourceConfig {
    BertResourceConfig {
        config: hf(model, "config.json").into(),
        weights: hf(model, "model.safetensors"),
        tokenizer: hf(model, "tokenizer.json"),
    }
}

impl BertResourceConfig {
    /// https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2
    pub fn mini_lm_l6_v2() -> Self {
        preset("sentence-transformers/all-MiniLM-L6-v2")
    }

    /// https://huggingface.co/google-bert/bert-large-uncased-whole-word-masking
    pub fn large_uncased_whole_word_masking() -> Self {
        preset("google-bert/bert-large-uncased-whole-word-masking")
    }

    /// https://huggingface.co/google-bert/bert-large-cased-whole-word-masking
    pub fn large_cased_whole_word_masking() -> Self {
        preset("google-bert/bert-large-cased-whole-word-masking")
    }
}
