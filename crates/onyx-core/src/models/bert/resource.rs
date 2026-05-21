pub struct BertResourceGroup {
    pub config: &'static str,
    pub weights: &'static str,
    pub vocab: &'static str,
}

impl BertResourceGroup {
    // pub async fn parse_config(&self) -> Result<super::BertConfig, error::ResourceError> {
    //     Ok(self.config.read().await?)
    // }
}

impl Default for BertResourceGroup {
    fn default() -> Self {
        Self {
            config: "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/config.json",
            weights: "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/model.safetensors",
            vocab: "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/vocab.json",
        }
    }
}
