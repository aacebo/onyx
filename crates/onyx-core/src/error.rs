pub enum Error {
    Message(String),
    Source(Box<dyn std::error::Error>),
}

impl Error {
    pub fn message(msg: impl ToString) -> Self {
        Self::Message(msg.to_string())
    }

    pub fn source(source: impl std::error::Error + 'static) -> Self {
        Self::Source(Box::new(source))
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(v) => write!(f, "{v}"),
            Self::Source(v) => write!(f, "{v}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Source(v) => Some(v.as_ref()),
            _ => None,
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::message(msg)
    }
}
