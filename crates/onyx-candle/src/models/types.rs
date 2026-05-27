#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Activation {
    Gelu,
    Relu,
    Silu,
    Tanh,
    GeluNew,
}

impl Activation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gelu => "gelu",
            Self::Relu => "relu",
            Self::Silu => "silu",
            Self::Tanh => "tanh",
            Self::GeluNew => "gelu_new",
        }
    }
}

impl std::fmt::Display for Activation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PositionEmbeddingType {
    Absolute,
    RelativeKey,
    RelativeKeyQuery,
}

impl PositionEmbeddingType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Absolute => "absolute",
            Self::RelativeKey => "relative_key",
            Self::RelativeKeyQuery => "relative_key_query",
        }
    }
}

impl std::fmt::Display for PositionEmbeddingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
