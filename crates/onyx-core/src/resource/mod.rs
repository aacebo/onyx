mod buffer;
mod local;
mod remote;

pub use buffer::*;
pub use local::*;
pub use remote::*;

pub trait ResourceProvider {
    fn get_resource(&self, key: &str) -> Option<Resource>;
}

pub trait ResourceResolver {
    fn resolve(&self, resource: &Resource) -> Result<Vec<u8>, crate::error::ResourceError>;
}

#[derive(Debug, Clone)]
pub enum Resource {
    Buffer(BufferResource),
    Local(LocalResource),
    Remote(RemoteResource),
}

impl Resource {
    pub fn buffer(content: impl Into<Vec<u8>>) -> Self {
        Self::Buffer(BufferResource {
            content: std::sync::Arc::new(content.into()),
        })
    }

    pub fn local(path: impl Into<std::path::PathBuf>) -> Self {
        Self::Local(LocalResource { path: path.into() })
    }

    pub fn remote(url: impl Into<String>, path: impl Into<std::path::PathBuf>) -> Self {
        Self::Remote(RemoteResource {
            url: url.into(),
            path: path.into(),
        })
    }

    pub fn path(&self) -> Option<&std::path::PathBuf> {
        match self {
            Self::Local(v) => Some(&v.path),
            Self::Remote(v) => Some(&v.path),
            _ => None,
        }
    }
}

impl From<BufferResource> for Resource {
    fn from(value: BufferResource) -> Self {
        Self::Buffer(value)
    }
}

impl From<LocalResource> for Resource {
    fn from(value: LocalResource) -> Self {
        Self::Local(value)
    }
}

impl From<RemoteResource> for Resource {
    fn from(value: RemoteResource) -> Self {
        Self::Remote(value)
    }
}
