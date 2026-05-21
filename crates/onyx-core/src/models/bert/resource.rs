use crate::{Resource, resources::RemoteResource};

pub struct BertResourceGroup {
    pub config: Box<dyn Resource>,
    pub weights: Box<dyn Resource>,
    pub vocab: Box<dyn Resource>,
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
