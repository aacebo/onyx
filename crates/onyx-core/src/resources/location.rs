#[derive(Clone, PartialEq, Eq)]
pub enum ResourceLocation {
    Local(std::path::PathBuf),
    Buffer(Vec<u8>),
    #[cfg(feature = "http")]
    Http(reqwest::Url),
}

impl std::fmt::Debug for ResourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for ResourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(v) => write!(f, "{}", v.display()),
            Self::Buffer(v) => write!(f, "<buffer:{}>", v.len()),
            #[cfg(feature = "http")]
            Self::Http(v) => write!(f, "{v}"),
        }
    }
}
