use std::sync::{Arc, LazyLock};

use async_trait::async_trait;

use crate::error;

#[async_trait]
pub trait ResourceProvider {
    async fn exists(&self) -> bool;
    async fn get(&self, key: &str) -> Option<&ResourceId>;
    async fn load(&self, key: &str) -> Result<Vec<u8>, error::ResourceError>;
}

#[derive(Clone, PartialEq, Eq)]
pub enum ResourceId {
    Local {
        path: std::path::PathBuf,
    },
    Remote {
        url: String,
    },
    #[cfg(feature = "huggingface")]
    HuggingFace {
        model_id: crate::model::ModelId, // "facebook/bart-large"
        filename: String,                // "model.onnx"
        revision: Option<String>,        // branch, tag, or commit
    },
}

impl ResourceId {
    pub fn local(path: impl Into<std::path::PathBuf>) -> Self {
        Self::Local { path: path.into() }
    }

    pub fn remote(url: impl Into<String>) -> Self {
        Self::Remote { url: url.into() }
    }

    #[cfg(feature = "huggingface")]
    pub fn huggingface(
        model_id: crate::model::ModelId,
        filename: String,
        revision: Option<String>,
    ) -> Self {
        Self::HuggingFace {
            model_id,
            filename,
            revision,
        }
    }
}

impl std::str::FromStr for ResourceId {
    type Err = error::ResourceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (scheme, rest) = value.split_once("://").ok_or(error::ResourceError::parse(
            "invalid resource id format: missing scheme",
        ))?;

        match scheme {
            "file" => match std::path::PathBuf::from_str(value) {
                Err(err) => Err(error::ResourceError::parse(err.to_string())),
                Ok(v) => Ok(Self::local(v)),
            },
            "http" | "https" => Ok(Self::remote(value)),
            "hf" => {
                let rest = value.strip_prefix("hf://").unwrap();

                let mut parts = rest.split('#');
                let model_id = parts.next().unwrap_or_default();
                let filename = parts.next().ok_or(error::ResourceError::parse(
                    "invalid resource id format: missing filename",
                ))?;
                let revision = parts.next();

                if parts.next().is_some() {
                    return Err(error::ResourceError::parse(
                        "invalid resource id format: too many fragments",
                    ));
                }

                if model_id.is_empty() {
                    return Err(error::ResourceError::parse(
                        "invalid resource id format: missing model id",
                    ));
                }

                if filename.is_empty() {
                    return Err(error::ResourceError::parse(
                        "invalid resource id format: missing filename",
                    ));
                }

                Ok(Self::HuggingFace {
                    model_id: model_id.parse().map_err(error::ResourceError::parse)?,
                    filename: filename.to_string(),
                    revision: revision
                        .filter(|value| !value.is_empty())
                        .map(ToOwned::to_owned),
                })
            }
            _ => Err(error::ResourceError::parse(
                "invalid resource id format: invalid or missing schema",
            )),
        }
    }
}

impl std::fmt::Debug for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local { path } => write!(f, "file://{}", path.as_path().display()),
            Self::Remote { url } => write!(f, "{url}"),
            Self::HuggingFace {
                model_id,
                filename,
                revision,
            } => {
                write!(f, "hf://{model_id}#{filename}")?;

                if let Some(rev) = revision {
                    write!(f, "#{rev}")?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    id: ResourceId,
    cache: std::path::PathBuf,
    data: Arc<LazyLock<Vec<u8>>>,
}

impl Resource {
    pub fn id(&self) -> &ResourceId {
        &self.id
    }

    pub fn cache(&self) -> &std::path::PathBuf {
        &self.cache
    }
}

impl AsRef<[u8]> for Resource {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl std::ops::Deref for Resource {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.data.as_ref()
    }
}
