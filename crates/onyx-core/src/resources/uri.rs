use crate::Error;
use crate::resources::Format;

#[derive(Clone, PartialEq, Eq)]
pub enum Uri {
    Local(std::path::PathBuf),
    Buffer(Format, Vec<u8>),
    #[cfg(feature = "http")]
    Http(url::Url),
}

impl Uri {
    pub fn local(path: impl Into<std::path::PathBuf>) -> Self {
        Self::Local(path.into())
    }

    pub fn buffer(format: Format, data: impl Into<Vec<u8>>) -> Self {
        Self::Buffer(format, data.into())
    }

    #[cfg(feature = "http")]
    pub fn http(url: url::Url) -> Self {
        Self::Http(url)
    }

    pub fn parse(uri: &str) -> Result<Self, Error> {
        let (scheme, next) = uri
            .split_once("://")
            .ok_or(Error::message("[resource::uri] must have a scheme"))?;

        Ok(match scheme {
            "file" => Self::local(std::path::PathBuf::from(next)),
            "data" => Self::buffer(Format::Unknown, next),
            "data:text/plain" => Self::buffer(Format::Text, next),
            #[cfg(feature = "json")]
            "data:application/json" => Self::buffer(Format::Json, next),
            #[cfg(feature = "http")]
            "http" | "https" => Self::Http(url::Url::parse(uri).map_err(|err| Error::message(err.to_string()))?),
            _ => return Err(Error::message("[resource::uri] unknown scheme")),
        })
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Local(v) => v.file_name()?.to_str(),
            Self::Buffer(_, _) => None,
            #[cfg(feature = "http")]
            Self::Http(v) => v.path_segments()?.last(),
        }
    }

    pub fn ext(&self) -> Option<&str> {
        let (_, ext) = self.name()?.rsplit_once('.')?;
        Some(ext)
    }

    pub fn format(&self) -> Format {
        match self {
            Self::Buffer(format, _) => *format,
            _ => Format::from_ext(self.ext().unwrap_or("unknown")),
        }
    }
}

impl std::str::FromStr for Uri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl std::fmt::Debug for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(v) => write!(f, "file://{}", v.display()),
            Self::Buffer(format, v) => write!(f, "<{}:{}>", format, v.len()),
            #[cfg(feature = "http")]
            Self::Http(v) => write!(f, "{v}"),
        }
    }
}

impl serde::Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let uri = String::deserialize(deserializer)?;
        let ur = Self::parse(&uri).map_err(|err| Error::custom(err))?;
        Ok(ur)
    }
}
