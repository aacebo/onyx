use onyx_core::BoxFuture;
use onyx_core::fs::FileSystem;

pub struct StdFileSystem(std::path::PathBuf);

impl From<&std::path::Path> for StdFileSystem {
    fn from(value: &std::path::Path) -> Self {
        Self(value.to_path_buf())
    }
}

impl FileSystem for StdFileSystem {
    fn exists(&self, path: &std::path::Path) -> BoxFuture<'_, std::io::Result<bool>> {
        let p = self.0.join(path);
        Box::pin(async move { std::fs::exists(p) })
    }

    fn metadata(&self, path: &std::path::Path) -> BoxFuture<'_, std::io::Result<std::fs::Metadata>> {
        let p = self.0.join(path);
        Box::pin(async move { std::fs::metadata(p) })
    }

    fn read(&self, path: &std::path::Path) -> BoxFuture<'_, std::io::Result<Vec<u8>>> {
        let p = self.0.join(path);
        Box::pin(async move { std::fs::read(p) })
    }

    fn write<'a>(&'a self, path: &std::path::Path, bytes: &'a [u8]) -> BoxFuture<'a, std::io::Result<()>> {
        let p = self.0.join(path);
        Box::pin(async move { std::fs::write(p, bytes) })
    }

    fn symlink(&self, src: &std::path::Path, dest: &std::path::Path) -> BoxFuture<'_, std::io::Result<()>> {
        let from = self.0.join(src);
        let to = self.0.join(dest);
        Box::pin(async move { _symlink(from, to) })
    }
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
