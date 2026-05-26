use std::fmt;

pub type Result<T> = std::result::Result<T, OnyxError>;

#[derive(Debug)]
pub enum OnyxError {
    Parse(ParseError),
    Resolve(ResolveError),
    Read(ReadError),
    Decode(DecodeError),
    Config(ConfigError),
    Tokenize(TokenizeError),
    Load(LoadError),
    Inference(InferenceError),
    Unsupported(UnsupportedError),
    Backend(Box<dyn std::error::Error + Send + Sync>),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, Clone)]
pub enum ParseError {
    Empty,
    InvalidModelId(String),
    InvalidUri(String),
}

#[derive(Debug)]
pub enum ResolveError {
    NotFound(String),
    Unavailable(String),
    UnsupportedScheme(String),
    PermissionDenied(String),
}

#[derive(Debug)]
pub enum ReadError {
    NotFound(String),
    Io(String),
}

#[derive(Debug)]
pub enum DecodeError {
    InvalidFormat(String),
    Json(String),
    Binary(String),
}

#[derive(Debug)]
pub enum ConfigError {
    MissingField(&'static str),
    InvalidField(&'static str),
    UnsupportedArchitecture(String),
    UnsupportedModelType(String),
}

#[derive(Debug)]
pub enum TokenizeError {
    InvalidInput(String),
    Backend(String),
}

#[derive(Debug)]
pub enum LoadError {
    MissingArtifact(String),
    InvalidWeights(String),
    InvalidConfig(String),
    Backend(String),
}

#[derive(Debug)]
pub enum InferenceError {
    InvalidInput(String),
    Backend(String),
}

#[derive(Debug)]
pub enum UnsupportedError {
    Backend(String),
}

impl fmt::Display for OnyxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "parse error: {e}"),
            Self::Resolve(e) => write!(f, "resolve error: {e}"),
            Self::Read(e) => write!(f, "read error: {e}"),
            Self::Decode(e) => write!(f, "decode error: {e}"),
            Self::Config(e) => write!(f, "config error: {e}"),
            Self::Tokenize(e) => write!(f, "tokenize error: {e}"),
            Self::Load(e) => write!(f, "load error: {e}"),
            Self::Inference(e) => write!(f, "inference error: {e}"),
            Self::Unsupported(e) => write!(f, "unsupported: {e}"),
            Self::Backend(e) => write!(f, "backend error: {e}"),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for OnyxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Backend(e) | Self::Other(e) => Some(&**e),
            _ => None,
        }
    }
}

impl serde::de::Error for OnyxError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        DecodeError::Json(msg.to_string()).into()
    }
}

impl serde::ser::Error for OnyxError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        DecodeError::Json(msg.to_string()).into()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty input"),
            Self::InvalidModelId(s) => write!(f, "invalid model id: {s}"),
            Self::InvalidUri(s) => write!(f, "invalid uri: {s}"),
        }
    }
}
impl std::error::Error for ParseError {}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(s) => write!(f, "not found: {s}"),
            Self::Unavailable(s) => write!(f, "unavailable: {s}"),
            Self::UnsupportedScheme(s) => write!(f, "unsupported scheme: {s}"),
            Self::PermissionDenied(s) => write!(f, "permission denied: {s}"),
        }
    }
}
impl std::error::Error for ResolveError {}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(s) => write!(f, "not found: {s}"),
            Self::Io(s) => write!(f, "io: {s}"),
        }
    }
}
impl std::error::Error for ReadError {}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(s) => write!(f, "invalid format: {s}"),
            Self::Json(s) => write!(f, "json: {s}"),
            Self::Binary(s) => write!(f, "binary: {s}"),
        }
    }
}
impl std::error::Error for DecodeError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField(s) => write!(f, "missing field: {s}"),
            Self::InvalidField(s) => write!(f, "invalid field: {s}"),
            Self::UnsupportedArchitecture(s) => write!(f, "unsupported architecture: {s}"),
            Self::UnsupportedModelType(s) => write!(f, "unsupported model type: {s}"),
        }
    }
}
impl std::error::Error for ConfigError {}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(s) => write!(f, "invalid input: {s}"),
            Self::Backend(s) => write!(f, "backend: {s}"),
        }
    }
}
impl std::error::Error for TokenizeError {}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingArtifact(s) => write!(f, "missing artifact: {s}"),
            Self::InvalidWeights(s) => write!(f, "invalid weights: {s}"),
            Self::InvalidConfig(s) => write!(f, "invalid config: {s}"),
            Self::Backend(s) => write!(f, "backend: {s}"),
        }
    }
}
impl std::error::Error for LoadError {}

impl fmt::Display for InferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(s) => write!(f, "invalid input: {s}"),
            Self::Backend(s) => write!(f, "backend: {s}"),
        }
    }
}
impl std::error::Error for InferenceError {}

impl fmt::Display for UnsupportedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backend(s) => write!(f, "backend: {s}"),
        }
    }
}
impl std::error::Error for UnsupportedError {}

impl From<ParseError> for OnyxError {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}
impl From<ResolveError> for OnyxError {
    fn from(e: ResolveError) -> Self {
        Self::Resolve(e)
    }
}
impl From<ReadError> for OnyxError {
    fn from(e: ReadError) -> Self {
        Self::Read(e)
    }
}
impl From<DecodeError> for OnyxError {
    fn from(e: DecodeError) -> Self {
        Self::Decode(e)
    }
}
impl From<ConfigError> for OnyxError {
    fn from(e: ConfigError) -> Self {
        Self::Config(e)
    }
}
impl From<TokenizeError> for OnyxError {
    fn from(e: TokenizeError) -> Self {
        Self::Tokenize(e)
    }
}
impl From<LoadError> for OnyxError {
    fn from(e: LoadError) -> Self {
        Self::Load(e)
    }
}
impl From<InferenceError> for OnyxError {
    fn from(e: InferenceError) -> Self {
        Self::Inference(e)
    }
}
impl From<UnsupportedError> for OnyxError {
    fn from(e: UnsupportedError) -> Self {
        Self::Unsupported(e)
    }
}

impl From<std::io::Error> for OnyxError {
    fn from(e: std::io::Error) -> Self {
        ReadError::Io(e.to_string()).into()
    }
}
impl From<std::str::Utf8Error> for OnyxError {
    fn from(e: std::str::Utf8Error) -> Self {
        DecodeError::Binary(e.to_string()).into()
    }
}
impl From<std::string::FromUtf8Error> for OnyxError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        DecodeError::Binary(e.to_string()).into()
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for OnyxError {
    fn from(e: serde_json::Error) -> Self {
        DecodeError::Json(e.to_string()).into()
    }
}

#[cfg(feature = "http")]
impl From<url::ParseError> for OnyxError {
    fn from(e: url::ParseError) -> Self {
        ParseError::InvalidUri(e.to_string()).into()
    }
}
