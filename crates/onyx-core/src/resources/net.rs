use super::{Resource, Uri};
use crate::Error;

pub trait Resolver: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// resolves a resource and returns
    /// its on-disk path.
    fn resolve(&self, uri: &Uri) -> impl Future<Output = Result<Resource, Self::Error>> + Send;
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
    type Error = Error;

    async fn resolve(&self, uri: &Uri) -> Result<Resource, Self::Error> {
        let mut resource = Resource::new(uri.clone());

        if let Some(path) = &self.dir {
            resource = resource.with_directory(path.clone());
        }

        let path = if let Some(p) = &resource.path {
            p
        } else {
            return Ok(resource);
        };

        if std::fs::exists(&path).map_err(Error::source)? {
            return Ok(resource);
        }

        match &resource.uri {
            Uri::Local(src) => {
                std::fs::copy(src, path).map_err(Error::source)?;
                Ok(resource)
            }
            Uri::Buffer(_, data) => {
                std::fs::write(&path, data).map_err(Error::source)?;
                Ok(resource)
            }
            #[allow(unreachable_patterns)]
            v => Err(Error::message(format!("unsupported resource type '{v}'"))),
        }
    }
}

pub(crate) mod internal {
    use super::*;
    use crate::internal::BoxFuture;

    pub trait AnyResolver: Send + Sync {
        fn resolve<'a>(&'a self, uri: &'a Uri) -> BoxFuture<'a, Result<Resource, Box<dyn std::error::Error + Send + Sync>>>;
    }

    impl<T, E> AnyResolver for T
    where
        T: Resolver<Error = E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        fn resolve<'a>(&'a self, uri: &'a Uri) -> BoxFuture<'a, Result<Resource, Box<dyn std::error::Error + Send + Sync>>> {
            Box::pin(async move {
                Resolver::resolve(self, uri)
                    .await
                    .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
            })
        }
    }
}
