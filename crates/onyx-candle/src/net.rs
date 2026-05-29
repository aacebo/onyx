pub struct HFRepository(hf_hub::api::tokio::ApiRepo);

impl onyx_core::net::Repository for HFRepository {
    fn exists(&self, path: &std::path::Path) -> onyx_core::BoxFuture<'_, onyx_core::Result<bool>> {
        let path = path.display().to_string();

        Box::pin(async move {
            match self.0.get(&path).await {
                Err(err) => match err {
                    hf_hub::api::tokio::ApiError::RequestError(err) => match err.status() {
                        Some(status) if status.as_u16() == 404 => Ok(false),
                        _ => Err(onyx_core::error::LoadError::Backend(format!("huggingface repository error: {}", err)).into()),
                    },
                    _ => Err(onyx_core::error::LoadError::Backend(format!("huggingface repository error: {}", err)).into()),
                },
                Ok(_) => Ok(true),
            }
        })
    }

    fn get(&self, _path: &std::path::Path) -> onyx_core::BoxFuture<'_, onyx_core::Result<onyx_core::net::Asset>> {
        todo!()
    }

    fn read(&self, _path: &std::path::Path) -> onyx_core::BoxFuture<'_, onyx_core::Result<onyx_core::net::AssetData>> {
        todo!()
    }

    fn copy(&self, _src: &std::path::Path, _dest: &std::path::Path) -> onyx_core::BoxFuture<'_, onyx_core::Result<u64>> {
        todo!()
    }
}
