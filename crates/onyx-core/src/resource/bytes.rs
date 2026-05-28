use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use crate::error::ParseError;
use crate::resource::Format;

#[non_exhaustive]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Encoding {
    Base64,
    #[default]
    Utf8,
}

impl Encoding {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Base64 => "base64",
            Self::Utf8 => "utf8",
        }
    }
}

impl std::fmt::Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Bytes {
    pub name: Option<String>,
    pub format: Format,
    pub encoding: Encoding,
    pub data: Vec<u8>,
}

impl Bytes {
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let (meta, payload) = s.split_once(',').unwrap_or(("", s));
        let mut params = meta.split(';');
        let mediatype = params.next().unwrap_or("");
        let mut name = None;
        let mut encoding = Encoding::Utf8;

        for param in params {
            match param.strip_prefix("name=") {
                Some(v) => name = Some(v.to_string()),
                None if param == "base64" => encoding = Encoding::Base64,
                None => {}
            }
        }

        let format = match mediatype {
            "text/plain" => Format::Text,
            "application/json" => Format::Json,
            _ => Format::Unknown,
        };

        let data = if encoding == Encoding::Base64 {
            STANDARD.decode(payload).map_err(|e| ParseError::InvalidUri(e.to_string()))?
        } else {
            payload.as_bytes().to_vec()
        };

        Ok(Self {
            name,
            format,
            encoding,
            data,
        })
    }

    pub const fn mime_type(&self) -> &'static str {
        match self.format {
            Format::Json => "application/json",
            _ => "text/plain",
        }
    }
}

impl std::str::FromStr for Bytes {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mediatype = self.mime_type();

        write!(f, "{mediatype}")?;

        if let Some(name) = &self.name {
            write!(f, ";name={}", name)?;
        }

        if self.encoding == Encoding::Base64 {
            write!(f, ";base64,{}", STANDARD.encode(&self.data))
        } else {
            write!(f, ",{}", String::from_utf8_lossy(&self.data))
        }
    }
}
