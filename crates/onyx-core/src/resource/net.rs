use super::{Resource, Uri};
use crate::BoxFuture;
use crate::error::ResolveError;

pub trait Resolver: Send + Sync {
    /// resolves a resource and returns
    /// its on-disk path.
    fn resolve<'a>(&'a self, uri: &'a Uri) -> BoxFuture<'a, crate::error::Result<Resource>>;
}

#[derive(Default)]
pub struct StdResolver {
    dir: Option<std::path::PathBuf>,
}

impl StdResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_directory(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.dir = Some(path.into());
        self
    }
}

impl Resolver for StdResolver {
    fn resolve<'a>(&'a self, uri: &'a Uri) -> BoxFuture<'a, crate::error::Result<Resource>> {
        Box::pin(async move {
            let mut resource = Resource::from_uri(uri.clone());

            if let Some(path) = &self.dir {
                resource.path = resource.uri.name().map(|name| path.join(name));
            }

            let path = if let Some(p) = &resource.path {
                p
            } else {
                return Ok(resource);
            };

            if let Ok(meta) = std::fs::metadata(path) {
                resource.size = meta.len();
            }

            if std::fs::exists(&path)? {
                return Ok(resource);
            }

            match &resource.uri {
                Uri::Local(src) => {
                    symlink(src, path)?;
                    Ok(resource)
                }
                Uri::Buffer(bytes) => {
                    std::fs::write(&path, &bytes.data)?;
                    Ok(resource)
                }
                #[allow(unreachable_patterns)]
                v => Err(ResolveError::UnsupportedScheme(v.to_string()).into()),
            }
        })
    }
}

pub fn symlink<From, To>(from: From, to: To) -> std::io::Result<()>
where
    From: AsRef<std::path::Path>,
    To: AsRef<std::path::Path>,
{
    _symlink(from, to)
}

#[cfg(windows)]
fn _symlink<From, To>(from: From, to: To) -> std::io::Result<()>
where
    From: AsRef<std::path::Path>,
    To: AsRef<std::path::Path>,
{
    std::os::windows::fs::symlink_file(from, to)
}

#[cfg(unix)]
fn _symlink<From, To>(from: From, to: To) -> std::io::Result<()>
where
    From: AsRef<std::path::Path>,
    To: AsRef<std::path::Path>,
{
    std::os::unix::fs::symlink(from, to)
}
