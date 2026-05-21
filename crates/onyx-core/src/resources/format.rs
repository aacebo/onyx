#[derive(Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceFormat {
    #[default]
    Unknown,
    SafeTensors,
    Onnx,
    Text,
    #[cfg(feature = "json")]
    Json,
}

impl ResourceFormat {
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
}

impl std::fmt::Debug for ResourceFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for ResourceFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
