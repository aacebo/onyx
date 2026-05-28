use onyx_core::BoxFuture;
use onyx_core::fs::File;

pub struct StdFile(std::path::PathBuf);

impl From<&std::path::Path> for StdFile {
    fn from(value: &std::path::Path) -> Self {
        Self(value.to_path_buf())
    }
}

impl File for StdFile {
    fn path(&self) -> &std::path::Path {
        &self.0
    }

    fn exists(&self) -> BoxFuture<'_, std::io::Result<bool>> {
        Box::pin(async move { std::fs::exists(&self.0) })
    }

    fn metadata(&self) -> BoxFuture<'_, std::io::Result<std::fs::Metadata>> {
        Box::pin(async move { self.0.metadata() })
    }

    fn read(&self) -> BoxFuture<'_, std::io::Result<Vec<u8>>> {
        Box::pin(async move { std::fs::read(&self.0) })
    }

    fn write<'a>(&'a self, bytes: &'a [u8]) -> BoxFuture<'a, std::io::Result<()>> {
        Box::pin(async move { std::fs::write(&self.0, bytes) })
    }

    fn symlink<'a>(&'a self, dest: &'a std::path::Path) -> BoxFuture<'a, std::io::Result<()>> {
        Box::pin(async move { _symlink(&self.0, dest) })
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
