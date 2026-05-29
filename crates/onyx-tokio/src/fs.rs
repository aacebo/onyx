use onyx_core::BoxFuture;
use onyx_core::fs::FileSystem;

pub struct TokioFileSystem(std::path::PathBuf);

impl From<&std::path::Path> for TokioFileSystem {
    fn from(value: &std::path::Path) -> Self {
        Self(value.to_path_buf())
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
