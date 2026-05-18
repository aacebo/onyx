#[cfg(feature = "huggingface")]
mod huggingface;
mod local;
mod remote;

#[cfg(feature = "huggingface")]
pub use huggingface::*;
pub use local::*;
pub use remote::*;

use async_trait::async_trait;

use crate::error;

#[async_trait]
pub trait ResourceProvider {
    async fn load(&self, id: &ResourceId) -> Result<Vec<u8>, error::ResourceError>;
}

#[derive(Clone, PartialEq, Eq)]
pub enum ResourceId {
    Local(LocalResource),
    Remote(RemoteResource),
    #[cfg(feature = "huggingface")]
    HuggingFace(HFResource),
}

impl From<LocalResource> for ResourceId {
    fn from(value: LocalResource) -> Self {
        Self::Local(value)
    }
}

impl From<RemoteResource> for ResourceId {
    fn from(value: RemoteResource) -> Self {
        Self::Remote(value)
    }
}

#[cfg(feature = "huggingface")]
impl From<HFResource> for ResourceId {
    fn from(value: HFResource) -> Self {
        Self::HuggingFace(value)
    }
}

impl std::str::FromStr for ResourceId {
    type Err = error::ResourceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (scheme, _) = value.split_once("://").ok_or(error::ResourceError::parse(
            "invalid resource id format: missing scheme",
        ))?;

        match scheme {
            "file" => Ok(LocalResource::from_str(value)?.into()),
            "http" | "https" => Ok(RemoteResource::from_str(value)?.into()),
            #[cfg(feature = "huggingface")]
            "hf" => Ok(HFResource::from_str(value)?.into()),
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
            Self::Local(v) => write!(f, "{v}"),
            Self::Remote(v) => write!(f, "{v}"),
            #[cfg(feature = "huggingface")]
            Self::HuggingFace(v) => write!(f, "{v}"),
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
        assert_eq!(LocalResource::new("/a/b").to_string(), "file:///a/b");
        assert_eq!(RemoteResource::new("https://x").to_string(), "https://x");
    }

    #[cfg(feature = "huggingface")]
    #[test]
    fn parse_hf_no_revision() {
        let id = ResourceId::from_str("hf://facebook/bart-large#model.onnx").expect("should parse");
        match &id {
            ResourceId::HuggingFace(v) => {
                assert_eq!(v.model_id().to_string(), "facebook/bart-large");
                assert_eq!(v.filename(), "model.onnx");
                assert_eq!(v.revision(), None);
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
            ResourceId::HuggingFace(v) => {
                assert_eq!(v.revision(), Some("main"));
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
            ResourceId::HuggingFace(v) => assert_eq!(v.revision(), None),
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
