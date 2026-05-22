use crate::Error;

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
    pub fn encode<T>(&self, value: &T) -> Result<Vec<u8>, Error>
    where
        T: serde::Serialize,
    {
        match self {
            #[cfg(feature = "json")]
            Self::Json => serde_json::to_vec(value).map_err(Error::source),
            v => Err(Error::message(format!("unsupported encode operation on format '{v}'"))),
        }
    }

    #[allow(unused)]
    pub fn decode<T>(&self, bytes: &[u8]) -> Result<T, Error>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        match self {
            #[cfg(feature = "json")]
            Self::Json => serde_json::from_slice(bytes).map_err(Error::source),
            v => Err(Error::message(format!("unsupported decode operation on format '{v}'"))),
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
