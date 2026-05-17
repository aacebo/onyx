#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum ResourceError {
    NotFound(String),
    IO(String),
}

impl ResourceError {
    pub fn not_found(target: impl Into<String>) -> Self {
        Self::NotFound(target.into())
    }

    pub fn io(message: impl Into<String>) -> Self {
        Self::IO(message.into())
    }
}

impl std::error::Error for ResourceError {}

impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(m) => write!(f, "[resource] error: \"{m}\" not found"),
            Self::IO(m) => write!(f, "[resource::io] error: {m}"),
        }
    }
}

impl From<std::io::Error> for ResourceError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value.to_string())
    }
}
