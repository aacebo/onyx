use onyx_core::resource;

use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum UriOrConfig {
    Uri(resource::Uri),
    Config(BertConfig),
}

impl From<resource::Uri> for UriOrConfig {
    fn from(value: resource::Uri) -> Self {
        Self::Uri(value)
    }
}

impl From<BertConfig> for UriOrConfig {
    fn from(value: BertConfig) -> Self {
        Self::Config(value)
    }
}
