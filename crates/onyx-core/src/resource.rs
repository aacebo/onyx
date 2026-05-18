use std::sync::{Arc, LazyLock};

use async_trait::async_trait;

use crate::error;

#[async_trait]
pub trait ResourceProvider {
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
            "file" => match std::path::PathBuf::from_str(rest) {
                Err(err) => Err(error::ResourceError::parse(err.to_string())),
                Ok(v) => Ok(Self::local(v)),
            },
            "http" | "https" => Ok(Self::remote(value)),
            #[cfg(feature = "huggingface")]
            "hf" => {
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
            #[cfg(feature = "huggingface")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn parse_err(value: &str) -> error::ResourceError {
        ResourceId::from_str(value).expect_err("should fail to parse")
    }

    fn assert_parse_err_contains(value: &str, needle: &str) {
        match parse_err(value) {
            error::ResourceError::Parse(m) => assert!(
                m.contains(needle),
                "for {value:?}: message {m:?} did not contain {needle:?}"
            ),
            other => panic!("expected Parse error for {value:?}, got {other:?}"),
        }
    }

    #[test]
    fn parse_missing_scheme() {
        assert_parse_err_contains("no-scheme-here", "missing scheme");
    }

    #[test]
    fn parse_unknown_scheme() {
        assert_parse_err_contains("ftp://host/x", "invalid or missing schema");
    }

    #[test]
    fn parse_file() {
        let id = ResourceId::from_str("file:///tmp/model.onnx").expect("should parse");
        assert!(matches!(id, ResourceId::Local { .. }));
    }

    #[test]
    fn parse_http_and_https() {
        let http = ResourceId::from_str("http://example.com/m").expect("should parse");
        assert!(matches!(http, ResourceId::Remote { .. }));
        assert_eq!(http.to_string(), "http://example.com/m");

        let https = ResourceId::from_str("https://example.com/m").expect("should parse");
        assert!(matches!(https, ResourceId::Remote { .. }));
        assert_eq!(https.to_string(), "https://example.com/m");
    }

    #[test]
    fn display_local_and_remote() {
        assert_eq!(ResourceId::local("/a/b").to_string(), "file:///a/b");
        assert_eq!(ResourceId::remote("https://x").to_string(), "https://x");
    }

    #[cfg(feature = "huggingface")]
    #[test]
    fn parse_hf_no_revision() {
        let id = ResourceId::from_str("hf://facebook/bart-large#model.onnx").expect("should parse");
        match &id {
            ResourceId::HuggingFace {
                model_id,
                filename,
                revision,
            } => {
                assert_eq!(model_id.to_string(), "facebook/bart-large");
                assert_eq!(filename, "model.onnx");
                assert_eq!(*revision, None);
            }
            other => panic!("expected HuggingFace, got {other:?}"),
        }
        assert_eq!(id.to_string(), "hf://facebook/bart-large#model.onnx");
    }

    #[cfg(feature = "huggingface")]
    #[test]
    fn parse_hf_with_revision() {
        let id =
            ResourceId::from_str("hf://facebook/bart-large#model.onnx#main").expect("should parse");
        match &id {
            ResourceId::HuggingFace { revision, .. } => {
                assert_eq!(revision.as_deref(), Some("main"));
            }
            other => panic!("expected HuggingFace, got {other:?}"),
        }
        assert_eq!(id.to_string(), "hf://facebook/bart-large#model.onnx#main");
    }

    #[cfg(feature = "huggingface")]
    #[test]
    fn parse_hf_empty_revision_fragment() {
        let id = ResourceId::from_str("hf://g/n#f#").expect("should parse");
        match id {
            ResourceId::HuggingFace { revision, .. } => assert_eq!(revision, None),
            other => panic!("expected HuggingFace, got {other:?}"),
        }
    }

    #[cfg(feature = "huggingface")]
    #[test]
    fn parse_hf_missing_filename() {
        assert_parse_err_contains("hf://facebook/bart-large", "missing filename");
    }

    #[cfg(feature = "huggingface")]
    #[test]
    fn parse_hf_empty_model_id() {
        assert_parse_err_contains("hf://#model.onnx", "missing model id");
    }

    #[cfg(feature = "huggingface")]
    #[test]
    fn parse_hf_too_many_fragments() {
        assert_parse_err_contains("hf://g/n#f#r#extra", "too many fragments");
    }

    #[cfg(feature = "huggingface")]
    #[test]
    fn parse_hf_bad_model_id() {
        // delegated ModelId parse fails: "nogroup" has no '/'
        assert!(matches!(
            parse_err("hf://nogroup#model.onnx"),
            error::ResourceError::Parse(_)
        ));
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
