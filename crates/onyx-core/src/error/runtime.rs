use crate::model::ModelFeature;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum RuntimeError {
    Load(String),
    Inference(String),
    IO(String),
    UnsupportedFeature(ModelFeature),
    ShapeMismatch { expected: String, got: String },
    MissingInput(String),
    Tokenizer(String),
}

impl std::error::Error for RuntimeError {}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Load(m) => write!(f, "[runtime::load] error: {m}"),
            Self::Inference(m) => write!(f, "[runtime::inference] error: {m}"),
            Self::IO(m) => write!(f, "[runtime::io] error: {m}"),
            Self::UnsupportedFeature(feat) => {
                write!(f, "[runtime] error: unsupported feature: {feat:?}")
            }
            Self::ShapeMismatch { expected, got } => write!(
                f,
                "[runtime] error: shape mismatch: expected {expected}, got {got}"
            ),
            Self::MissingInput(name) => {
                write!(f, "[runtime] error: missing input: {name}")
            }
            Self::Tokenizer(m) => write!(f, "[runtime::tokenizer] error: {m}"),
        }
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value.to_string())
    }
}
