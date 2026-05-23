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
