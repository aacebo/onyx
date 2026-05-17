pub trait ResourceProvider {
    fn get_resource(&self, key: &str) -> Option<Resource>;
}

pub trait ResourceLoader {
    fn load_resource(&self, resource: &Resource) -> Result<Vec<u8>, crate::error::ResourceError>;
}

#[derive(Debug, Clone)]
pub enum Resource {
    Buffer {
        content: std::sync::Arc<Vec<u8>>,
    },
    Local {
        path: std::path::PathBuf,
    },
    Remote {
        url: String,
        path: std::path::PathBuf,
    },
}

impl Resource {
    pub fn buffer(content: impl Into<Vec<u8>>) -> Self {
        Self::Buffer {
            content: std::sync::Arc::new(content.into()),
        }
    }

    pub fn local(path: impl Into<std::path::PathBuf>) -> Self {
        Self::Local { path: path.into() }
    }

    pub fn remote(url: impl Into<String>, path: impl Into<std::path::PathBuf>) -> Self {
        Self::Remote {
            url: url.into(),
            path: path.into(),
        }
    }

    pub fn path(&self) -> Option<&std::path::PathBuf> {
        match self {
            Self::Local { path } => Some(path),
            Self::Remote { path, .. } => Some(path),
            _ => None,
        }
    }
}
