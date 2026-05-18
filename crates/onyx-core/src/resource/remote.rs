use crate::error;

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RemoteResource(String);

impl RemoteResource {
    pub fn new(url: impl Into<String>) -> Self {
        Self(url.into())
    }
}

impl std::ops::Deref for RemoteResource {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for RemoteResource {
    type Err = error::ResourceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}

impl std::fmt::Debug for RemoteResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for RemoteResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
