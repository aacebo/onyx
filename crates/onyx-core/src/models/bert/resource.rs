use crate::resources;

pub struct BertResourceGroup {
    pub config: resources::Uri,
    pub weights: resources::Uri,
    pub vocab: resources::Uri,
    pub tokenizer_config: resources::Uri,
}

impl BertResourceGroup {
    pub fn base_uncased() -> Self {
        Self {
            config: "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/config.json"
                .parse()
                .unwrap(),
            weights: "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/model.safetensors"
                .parse()
                .unwrap(),
            vocab: "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/vocab.txt"
                .parse()
                .unwrap(),
            tokenizer_config: "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/tokenizer_config.json"
                .parse()
                .unwrap(),
        }
    }

    pub fn base_cased() -> Self {
        Self {
            config: "https://huggingface.co/google-bert/bert-base-cased/resolve/main/config.json"
                .parse()
                .unwrap(),
            weights: "https://huggingface.co/google-bert/bert-base-cased/resolve/main/model.safetensors"
                .parse()
                .unwrap(),
            vocab: "https://huggingface.co/google-bert/bert-base-cased/resolve/main/vocab.txt"
                .parse()
                .unwrap(),
            tokenizer_config: "https://huggingface.co/google-bert/bert-base-cased/resolve/main/tokenizer_config.json"
                .parse()
                .unwrap(),
        }
    }

    pub fn large_uncased() -> Self {
        Self {
            config: "https://huggingface.co/google-bert/bert-large-uncased/resolve/main/config.json"
                .parse()
                .unwrap(),
            weights: "https://huggingface.co/google-bert/bert-large-uncased/resolve/main/model.safetensors"
                .parse()
                .unwrap(),
            vocab: "https://huggingface.co/google-bert/bert-large-uncased/resolve/main/vocab.txt"
                .parse()
                .unwrap(),
            tokenizer_config: "https://huggingface.co/google-bert/bert-large-uncased/resolve/main/tokenizer_config.json"
                .parse()
                .unwrap(),
        }
    }

    pub fn large_cased() -> Self {
        Self {
            config: "https://huggingface.co/google-bert/bert-large-cased/resolve/main/config.json"
                .parse()
                .unwrap(),
            weights: "https://huggingface.co/google-bert/bert-large-cased/resolve/main/model.safetensors"
                .parse()
                .unwrap(),
            vocab: "https://huggingface.co/google-bert/bert-large-cased/resolve/main/vocab.txt"
                .parse()
                .unwrap(),
            tokenizer_config: "https://huggingface.co/google-bert/bert-large-cased/resolve/main/tokenizer_config.json"
                .parse()
                .unwrap(),
        }
    }

    pub fn base_chinese() -> Self {
        Self {
            config: "https://huggingface.co/google-bert/bert-base-chinese/resolve/main/config.json"
                .parse()
                .unwrap(),
            weights: "https://huggingface.co/google-bert/bert-base-chinese/resolve/main/model.safetensors"
                .parse()
                .unwrap(),
            vocab: "https://huggingface.co/google-bert/bert-base-chinese/resolve/main/vocab.txt"
                .parse()
                .unwrap(),
            tokenizer_config: "https://huggingface.co/google-bert/bert-base-chinese/resolve/main/tokenizer_config.json"
                .parse()
                .unwrap(),
        }
    }

    pub fn base_multilingual_cased() -> Self {
        Self {
            config: "https://huggingface.co/google-bert/bert-base-multilingual-cased/resolve/main/config.json"
                .parse()
                .unwrap(),
            weights: "https://huggingface.co/google-bert/bert-base-multilingual-cased/resolve/main/model.safetensors"
                .parse()
                .unwrap(),
            vocab: "https://huggingface.co/google-bert/bert-base-multilingual-cased/resolve/main/vocab.txt"
                .parse()
                .unwrap(),
            tokenizer_config:
                "https://huggingface.co/google-bert/bert-base-multilingual-cased/resolve/main/tokenizer_config.json"
                    .parse()
                    .unwrap(),
        }
    }

    pub fn large_uncased_whole_word_masking() -> Self {
        Self {
            config: "https://huggingface.co/google-bert/bert-large-uncased-whole-word-masking/resolve/main/config.json"
                .parse()
                .unwrap(),
            weights: "https://huggingface.co/google-bert/bert-large-uncased-whole-word-masking/resolve/main/model.safetensors"
                .parse()
                .unwrap(),
            vocab: "https://huggingface.co/google-bert/bert-large-uncased-whole-word-masking/resolve/main/vocab.txt"
                .parse()
                .unwrap(),
            tokenizer_config:
                "https://huggingface.co/google-bert/bert-large-uncased-whole-word-masking/resolve/main/tokenizer_config.json"
                    .parse()
                    .unwrap(),
        }
    }

    pub fn large_cased_whole_word_masking() -> Self {
        Self {
            config: "https://huggingface.co/google-bert/bert-large-cased-whole-word-masking/resolve/main/config.json"
                .parse()
                .unwrap(),
            weights: "https://huggingface.co/google-bert/bert-large-cased-whole-word-masking/resolve/main/model.safetensors"
                .parse()
                .unwrap(),
            vocab: "https://huggingface.co/google-bert/bert-large-cased-whole-word-masking/resolve/main/vocab.txt"
                .parse()
                .unwrap(),
            tokenizer_config:
                "https://huggingface.co/google-bert/bert-large-cased-whole-word-masking/resolve/main/tokenizer_config.json"
                    .parse()
                    .unwrap(),
        }
    }
}

impl Default for BertResourceGroup {
    fn default() -> Self {
        Self::base_uncased()
    }
}
