use async_trait::async_trait;

use crate::{Model, error, model};

pub struct HFModelHub {
    _api: hf_hub::Cache,
}

#[async_trait]
impl model::ModelProvider for HFModelHub {
    async fn load(&self, id: &model::ModelId) -> Result<Model, error::ModelError> {
        todo!()
    }
}
