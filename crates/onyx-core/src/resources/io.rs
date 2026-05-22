use super::{Resource, Uri};

pub trait Reader: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    fn read(&self, resource: &Resource) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send;
}

#[derive(Default, Copy, Clone)]
pub struct StdReader;

impl Reader for StdReader {
    type Error = std::io::Error;

    async fn read(&self, resource: &Resource) -> Result<Vec<u8>, Self::Error> {
        if let Some(path) = &resource.path {
            std::fs::read(path)
        } else if let Uri::Buffer(_, data) = &resource.uri {
            Ok(data.clone())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file not found, no path provided",
            ))
        }
    }
}

pub(crate) mod internal {
    use super::*;
    use crate::internal::BoxFuture;

    pub trait AnyReader: Send + Sync {
        fn read<'a>(&'a self, resource: &'a Resource)
        -> BoxFuture<'a, Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>>;
    }

    impl<T, E> AnyReader for T
    where
        T: Reader<Error = E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        fn read<'a>(
            &'a self,
            resource: &'a Resource,
        ) -> BoxFuture<'a, Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>> {
            Box::pin(async move {
                Reader::read(self, resource)
                    .await
                    .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
            })
        }
    }
}
