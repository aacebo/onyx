use crate::BoxFuture;

pub trait File {
    fn path(&self) -> &std::path::Path;
    fn exists(&self) -> BoxFuture<'_, std::io::Result<bool>>;
    fn metadata(&self) -> BoxFuture<'_, std::io::Result<std::fs::Metadata>>;
    fn read(&self) -> BoxFuture<'_, std::io::Result<Vec<u8>>>;
    fn write<'a>(&'a self, bytes: &'a [u8]) -> BoxFuture<'a, std::io::Result<()>>;
    fn symlink<'a>(&'a self, dest: &'a std::path::Path) -> BoxFuture<'a, std::io::Result<()>>;
}

pub trait FileSystem {
    fn exists(&self, path: &std::path::Path) -> BoxFuture<'_, std::io::Result<bool>>;
    fn metadata(&self, path: &std::path::Path) -> BoxFuture<'_, std::io::Result<std::fs::Metadata>>;
    fn read(&self, path: &std::path::Path) -> BoxFuture<'_, std::io::Result<Vec<u8>>>;
    fn write<'a>(&'a self, path: &std::path::Path, bytes: &'a [u8]) -> BoxFuture<'a, std::io::Result<()>>;
    fn symlink<'a>(&'a self, src: &std::path::Path, dest: &'a std::path::Path) -> BoxFuture<'a, std::io::Result<()>>;
}
