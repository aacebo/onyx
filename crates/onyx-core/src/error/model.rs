#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum ModelError {
    NotFound(String),
    Parse(String),
    Tokenize(String),
}

impl ModelError {
    pub fn not_found(target: impl Into<String>) -> Self {
        Self::NotFound(target.into())
    }

    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse(message.into())
    }

    pub fn tokenize(message: impl Into<String>) -> Self {
        Self::Tokenize(message.into())
    }
}

impl std::error::Error for ModelError {}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(m) => write!(f, "[model] error: \"{m}\" not found"),
            Self::Parse(m) => write!(f, "[model::parse] error: {m}"),
            Self::Tokenize(m) => write!(f, "[model::tokenize] error: {m}"),
        }
    }
}

impl From<std::string::ParseError> for ModelError {
    fn from(value: std::string::ParseError) -> Self {
        Self::Parse(value.to_string())
    }
}
