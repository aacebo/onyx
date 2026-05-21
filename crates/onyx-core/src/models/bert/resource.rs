use crate::{Resource, error, resources::RemoteResource};

pub struct BertResourceGroup {
    pub config: Box<dyn Resource>,
    pub weights: Box<dyn Resource>,
    pub vocab: Box<dyn Resource>,
}

impl BertResourceGroup {
    pub async fn parse_config(&self) -> Result<super::BertConfig, error::ResourceError> {
        Ok(self.config.read().await?)
    }
}

impl Default for BertResourceGroup {
    fn default() -> Self {
        Self {
            config: Box::new(RemoteResource::parse(
                "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/config.json",
            )
            .unwrap()),
            weights: Box::new(RemoteResource::parse(
                "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/model.safetensors",
            )
            .unwrap()),
            vocab:     Box::new(RemoteResource::parse(
                "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/vocab.json",
            )
            .unwrap()),
        }
    }
}
