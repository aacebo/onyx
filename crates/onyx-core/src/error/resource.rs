#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum ResourceError {
    Api(String),
    NotFound(String),
    Parse(String),
    IO(String),
}

impl ResourceError {
    pub fn api(message: impl std::fmt::Display) -> Self {
        Self::Api(message.to_string())
    }

    pub fn not_found(target: impl std::fmt::Display) -> Self {
        Self::NotFound(target.to_string())
    }

    pub fn parse(message: impl std::fmt::Display) -> Self {
        Self::Parse(message.to_string())
    }

    pub fn io(message: impl std::fmt::Display) -> Self {
        Self::IO(message.to_string())
    }
}

impl std::error::Error for ResourceError {}

impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Api(m) => write!(f, "[resource::api] error: {m}"),
            Self::NotFound(m) => write!(f, "[resource] error: \"{m}\" not found"),
            Self::Parse(m) => write!(f, "[resource::parse] error: {m}"),
            Self::IO(m) => write!(f, "[resource::io] error: {m}"),
        }
    }
}

impl From<std::io::Error> for ResourceError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value.to_string())
    }
}
