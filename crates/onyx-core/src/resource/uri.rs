use crate::error::ParseError;
use crate::resource::{Bytes, Format};

#[derive(Clone, PartialEq, Eq)]
pub enum Uri {
    Local(std::path::PathBuf),
    Buffer(Bytes),
    Http(url::Url),
}

impl Uri {
    pub fn local(path: impl Into<std::path::PathBuf>) -> Self {
        Self::Local(path.into())
    }

    pub fn buffer(bytes: impl Into<Bytes>) -> Self {
        Self::Buffer(bytes.into())
    }

    pub fn http(url: url::Url) -> Self {
        Self::Http(url)
    }

    pub fn parse(uri: &str) -> crate::error::Result<Self> {
        let (scheme, next) = uri.split_once("://").ok_or_else(|| ParseError::InvalidUri(uri.to_string()))?;

        Ok(match scheme {
            "file" => Self::local(std::path::PathBuf::from(next)),
            "http" | "https" => Self::Http(url::Url::parse(uri)?),
            "data" => Self::Buffer(Bytes::parse(next)?),
            _ => return Err(ParseError::InvalidUri(uri.to_string()).into()),
        })
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Local(v) => v.file_name()?.to_str(),
            Self::Buffer(v) => v.name.as_deref(),
            Self::Http(v) => v.path_segments()?.last(),
        }
    }

    pub fn ext(&self) -> Option<&str> {
        let (_, ext) = self.name()?.rsplit_once('.')?;
        Some(ext)
    }

    pub fn format(&self) -> Format {
        match self.ext() {
            None => Format::default(),
            Some(ext) => Format::from_ext(ext),
        }
    }
}

impl std::str::FromStr for Uri {
    type Err = crate::OnyxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl std::fmt::Debug for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(v) => write!(f, "file://{}", v.display()),
            Self::Buffer(v) => write!(f, "data://{v}"),
            Self::Http(v) => write!(f, "{v}"),
        }
    }
}

impl serde::Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let uri = String::deserialize(deserializer)?;
        let ur = Self::parse(&uri).map_err(|err| Error::custom(err))?;
        Ok(ur)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::Encoding;

    fn buffer(format: Format, encoding: Encoding, data: impl Into<Vec<u8>>) -> Uri {
        Uri::Buffer(Bytes {
            name: None,
            format,
            encoding,
            data: data.into(),
        })
    }

    fn named_buffer(name: &str, format: Format, encoding: Encoding, data: impl Into<Vec<u8>>) -> Uri {
        Uri::Buffer(Bytes {
            name: Some(name.to_string()),
            format,
            encoding,
            data: data.into(),
        })
    }

    #[test]
    fn parses_plain_text_data() {
        assert_eq!(
            Uri::parse("data://text/plain,hello").unwrap(),
            buffer(Format::Text, Encoding::Utf8, b"hello".to_vec())
        );
    }

    #[test]
    fn parses_base64_json_data() {
        assert_eq!(
            Uri::parse("data://application/json;base64,e30=").unwrap(),
            buffer(Format::Json, Encoding::Base64, b"{}".to_vec())
        );
    }

    #[test]
    fn parses_data_without_mediatype() {
        assert_eq!(
            Uri::parse("data://hello").unwrap(),
            buffer(Format::Unknown, Encoding::Utf8, b"hello".to_vec())
        );
    }

    #[test]
    fn rejects_invalid_base64() {
        assert!(matches!(
            Uri::parse("data://;base64,not valid!"),
            Err(crate::OnyxError::Parse(ParseError::InvalidUri(_)))
        ));
    }

    #[test]
    fn parses_name_param() {
        assert_eq!(
            Uri::parse("data://application/json;name=config.json;base64,e30=").unwrap(),
            named_buffer("config.json", Format::Json, Encoding::Base64, b"{}".to_vec())
        );
    }

    #[test]
    fn round_trips_text_json_and_binary() {
        let cases = [
            buffer(Format::Text, Encoding::Utf8, b"hello world".to_vec()),
            buffer(Format::Json, Encoding::Base64, b"{\"a\":1}".to_vec()),
            buffer(Format::Json, Encoding::Base64, vec![0u8, 159, 146, 150, 255]),
            named_buffer("config.json", Format::Json, Encoding::Base64, b"{}".to_vec()),
            named_buffer("notes.txt", Format::Text, Encoding::Utf8, b"hi".to_vec()),
        ];

        for uri in cases {
            assert_eq!(Uri::parse(&uri.to_string()).unwrap(), uri);
        }
    }

    #[test]
    fn other_schemes_unchanged() {
        assert!(matches!(Uri::parse("file:///tmp/x.json").unwrap(), Uri::Local(_)));
        assert!(matches!(
            Uri::parse("https://huggingface.co/m/resolve/main/f.safetensors").unwrap(),
            Uri::Http(_)
        ));
    }
}
