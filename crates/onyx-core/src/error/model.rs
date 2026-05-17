#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum ModelError {
    NotFound(String),
    Parse(String),
}

impl ModelError {
    pub fn not_found(target: impl Into<String>) -> Self {
        Self::NotFound(target.into())
    }

    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse(message.into())
    }
}

impl std::error::Error for ModelError {}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(m) => write!(f, "[resource] error: \"{m}\" not found"),
            Self::Parse(m) => write!(f, "[resource::model] error: {m}"),
        }
    }
}

impl From<std::string::ParseError> for ModelError {
    fn from(value: std::string::ParseError) -> Self {
        Self::Parse(value.to_string())
    }
}
