use async_trait::async_trait;

use crate::{error, resource};

pub struct HFResourceHub {
    api: hf_hub::api::tokio::Api,
}

#[async_trait]
impl resource::ResourceProvider for HFResourceHub {
    async fn load(&self, id: &resource::ResourceId) -> Result<Vec<u8>, error::ResourceError> {
        if let resource::ResourceId::HuggingFace(resource) = id {
            let source = self.api.model(resource.model_id().to_string());
            let path = source
                .get(resource.filename())
                .await
                .map_err(error::ResourceError::api)?;

            return Ok(std::fs::read(path).map_err(error::ResourceError::io)?);
        }

        Err(error::ResourceError::NotFound(id.to_string()))
    }
}
