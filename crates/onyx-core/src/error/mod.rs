mod config;
mod decode;
mod inference;
mod load;
mod parse;
mod read;
mod resolve;
mod tokenize;
mod unsupported;

pub use config::ConfigError;
pub use decode::DecodeError;
pub use inference::InferenceError;
pub use load::LoadError;
pub use parse::ParseError;
pub use read::ReadError;
pub use resolve::ResolveError;
pub use tokenize::TokenizeError;
pub use unsupported::UnsupportedError;

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

impl std::fmt::Display for OnyxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        DecodeError::Json(msg.to_string()).into()
    }
}

impl serde::ser::Error for OnyxError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        DecodeError::Json(msg.to_string()).into()
    }
}

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

impl From<url::ParseError> for OnyxError {
    fn from(e: url::ParseError) -> Self {
        ParseError::InvalidUri(e.to_string()).into()
    }
}
