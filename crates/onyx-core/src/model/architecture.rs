#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelArchitecture {
    Bart,
    Bert,
    Roberta,
    DistilBert,
    Deberta,
    Gpt2,
    Llama,
    Mistral,
    T5,
    Clip,
    Whisper,
    #[default]
    #[serde(other)]
    Unknown,
}

impl ModelArchitecture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bart => "bart",
            Self::Bert => "bert",
            Self::Roberta => "roberta",
            Self::DistilBert => "distilbert",
            Self::Deberta => "deberta",
            Self::Gpt2 => "gpt2",
            Self::Llama => "llama",
            Self::Mistral => "mistral",
            Self::T5 => "t5",
            Self::Clip => "clip",
            Self::Whisper => "whisper",
            _ => "??",
        }
    }
}

impl std::fmt::Display for ModelArchitecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
