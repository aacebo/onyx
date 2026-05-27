#[non_exhaustive]
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    SafeTensors,
    Onnx,
    Text,
    Json,
    #[default]
    #[serde(other)]
    Unknown,
}

impl Format {
    pub fn from_ext(ext: impl AsRef<str>) -> Self {
        match ext.as_ref() {
            "safetensors" => Self::SafeTensors,
            "onnx" => Self::Onnx,
            "txt" => Self::Text,
            "json" => Self::Json,
            _ => Self::Unknown,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeTensors => "safetensors",
            Self::Onnx => "onnx",
            Self::Text => "text",
            Self::Json => "json",
            _ => "??",
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
