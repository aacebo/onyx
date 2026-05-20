use async_trait::async_trait;

use crate::{Resource, error, model};

#[derive(Clone, PartialEq, Eq)]
pub struct HFResource {
    model_id: model::ModelId,
    filename: String,
    path: std::path::PathBuf,
}

impl HFResource {
    pub fn parse(url: impl AsRef<str>) -> Result<Self, error::ResourceError> {
        let parsed = url::Url::parse(url.as_ref()).map_err(error::ResourceError::parse)?;

        let host =
            parsed
                .host_str()
                .filter(|s| !s.is_empty())
                .ok_or(error::ResourceError::parse(
                    "invalid hf url: missing model id",
                ))?;

        let mut segments: Vec<&str> = parsed
            .path_segments()
            .map(|s| s.filter(|p| !p.is_empty()).collect())
            .unwrap_or_default();

        let filename = segments
            .pop()
            .filter(|s| !s.is_empty())
            .ok_or(error::ResourceError::parse(
                "invalid hf url: missing filename",
            ))?
            .to_string();

        let model_id: model::ModelId = if segments.is_empty() {
            host.to_string()
        } else {
            format!("{}/{}", host, segments.join("/"))
        }
        .parse()
        .map_err(error::ResourceError::parse)?;

        let query: std::collections::HashMap<_, _> = parsed.query_pairs().into_owned().collect();

        let path = if let Some(p) = query.get("path") {
            std::path::PathBuf::from(p).join(&filename)
        } else {
            std::env::temp_dir().join(format!("onyx/{}", &filename))
        };

        Ok(Self {
            model_id,
            filename,
            path,
        })
    }

    pub fn model_id(&self) -> &model::ModelId {
        &self.model_id
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }
}

impl std::str::FromStr for HFResource {
    type Err = error::ResourceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl std::fmt::Debug for HFResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for HFResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hf://{}/{}", &self.model_id, &self.filename)
    }
}

impl serde::Serialize for HFResource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for HFResource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[async_trait]
impl Resource for HFResource {
    async fn read(&self) -> Result<std::path::PathBuf, error::ResourceError> {
        let path = hf_hub::api::tokio::Api::new()
            .map_err(error::ResourceError::api)?
            .model(self.model_id.to_string())
            .get(&self.filename)
            .await
            .map_err(error::ResourceError::api)?;

        Ok(path)
    }
}
