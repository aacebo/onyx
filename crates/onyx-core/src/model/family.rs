#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelFamily {
    Encoder,
    Decoder,
    EncoderDecoder,
    SentenceTransformer,
    Vision,
    Audio,
    Multimodal,
    #[default]
    #[serde(other)]
    Unknown,
}

impl ModelFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Encoder => "encoder",
            Self::Decoder => "decoder",
            Self::EncoderDecoder => "encoder-decoder",
            Self::SentenceTransformer => "sentence-transformer",
            Self::Vision => "vision",
            Self::Audio => "audio",
            Self::Multimodal => "multi-modal",
            _ => "??",
        }
    }
}

impl std::fmt::Display for ModelFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
