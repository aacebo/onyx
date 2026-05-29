use onyx_core::BoxFuture;
use onyx_core::fs::FileSystem;
use onyx_core::net::Repository;

pub struct TokioFileSystem(std::path::PathBuf);

impl<T: AsRef<std::path::Path>> From<T> for TokioFileSystem {
    fn from(value: T) -> Self {
        Self(value.as_ref().to_path_buf())
    }
}

impl FileSystem for TokioFileSystem {
    fn exists(&self, path: &std::path::Path) -> BoxFuture<'_, std::io::Result<bool>> {
        let p = self.0.join(path);
        Box::pin(async move { tokio::fs::try_exists(p).await })
    }

    fn metadata(&self, path: &std::path::Path) -> BoxFuture<'_, std::io::Result<std::fs::Metadata>> {
        let p = self.0.join(path);
        Box::pin(async move { tokio::fs::metadata(p).await })
    }

    fn read(&self, path: &std::path::Path) -> BoxFuture<'_, std::io::Result<Vec<u8>>> {
        let p = self.0.join(path);
        Box::pin(async move { tokio::fs::read(p).await })
    }

    fn write<'a>(&'a self, path: &std::path::Path, bytes: &'a [u8]) -> BoxFuture<'a, std::io::Result<()>> {
        let p = self.0.join(path);
        Box::pin(async move { tokio::fs::write(p, bytes).await })
    }

    fn symlink(&self, src: &std::path::Path, dest: &std::path::Path) -> BoxFuture<'_, std::io::Result<()>> {
        let from = self.0.join(src);
        let to = self.0.join(dest);
        Box::pin(async move { _symlink(from, to).await })
    }
}

impl Repository for TokioFileSystem {
    fn exists(&self, path: &std::path::Path) -> BoxFuture<'_, onyx_core::Result<bool>> {
        let p = self.0.join(path);
        Box::pin(async move { Ok(tokio::fs::try_exists(p).await?) })
    }

    fn get(&self, _path: &std::path::Path) -> BoxFuture<'_, onyx_core::Result<onyx_core::net::Asset>> {
        todo!()
    }

    fn read(&self, _path: &std::path::Path) -> BoxFuture<'_, onyx_core::Result<onyx_core::net::AssetData>> {
        todo!()
    }

    fn copy(&self, _src: &std::path::Path, _dest: &std::path::Path) -> BoxFuture<'_, onyx_core::Result<u64>> {
        todo!()
    }
}

#[cfg(windows)]
async fn _symlink<From, To>(from: From, to: To) -> std::io::Result<()>
where
    From: AsRef<std::path::Path>,
    To: AsRef<std::path::Path>,
{
    tokio::fs::symlink_file(from, to).await
}

#[cfg(unix)]
async fn _symlink<From, To>(from: From, to: To) -> std::io::Result<()>
where
    From: AsRef<std::path::Path>,
    To: AsRef<std::path::Path>,
{
    tokio::fs::symlink(from, to).await
}
