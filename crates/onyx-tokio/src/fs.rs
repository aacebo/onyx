pub async fn symlink<From, To>(from: From, to: To) -> std::io::Result<()>
where
    From: AsRef<std::path::Path>,
    To: AsRef<std::path::Path>,
{
    _symlink(from, to).await
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
