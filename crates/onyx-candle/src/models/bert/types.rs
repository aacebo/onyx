use onyx_core::resources;

use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum UriOrConfig {
    Uri(resources::Uri),
    Config(BertConfig),
}

impl From<resources::Uri> for UriOrConfig {
    fn from(value: resources::Uri) -> Self {
        Self::Uri(value)
    }
}

impl From<BertConfig> for UriOrConfig {
    fn from(value: BertConfig) -> Self {
        Self::Config(value)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum UriOrTokenizerConfig {
    Uri(resources::Uri),
    Config(BertTokenizerConfig),
}

impl From<resources::Uri> for UriOrTokenizerConfig {
    fn from(value: resources::Uri) -> Self {
        Self::Uri(value)
    }
}

impl From<BertTokenizerConfig> for UriOrTokenizerConfig {
    fn from(value: BertTokenizerConfig) -> Self {
        Self::Config(value)
    }
}
