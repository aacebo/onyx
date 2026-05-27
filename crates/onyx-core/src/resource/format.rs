use crate::error::DecodeError;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    #[default]
    Unknown,
    SafeTensors,
    Onnx,
    Text,
    #[cfg(feature = "json")]
    Json,
}

impl Format {
    pub fn from_ext(ext: impl AsRef<str>) -> Self {
        match ext.as_ref() {
            "safetensors" => Self::SafeTensors,
            "onnx" => Self::Onnx,
            "txt" => Self::Text,
            #[cfg(feature = "json")]
            "json" => Self::Json,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::SafeTensors => "safe_tensors",
            Self::Onnx => "onnx",
            Self::Text => "text",
            #[cfg(feature = "json")]
            Self::Json => "json",
        }
    }

    #[allow(unused)]
    pub fn encode<T>(&self, value: &T) -> crate::error::Result<Vec<u8>>
    where
        T: serde::Serialize,
    {
        match self {
            #[cfg(feature = "json")]
            Self::Json => Ok(serde_json::to_vec(value)?),
            v => Err(DecodeError::InvalidFormat(format!("unsupported encode operation on format '{v}'")).into()),
        }
    }

    #[allow(unused)]
    pub fn decode<T>(&self, bytes: &[u8]) -> crate::error::Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        match self {
            #[cfg(feature = "json")]
            Self::Json => Ok(serde_json::from_slice(bytes)?),
            v => Err(DecodeError::InvalidFormat(format!("unsupported decode operation on format '{v}'")).into()),
        }
    }
}

impl std::fmt::Debug for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
