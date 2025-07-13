use {
    crate::{IoError, Result},
    std::path::Path,
    tokio::fs,
};

pub async fn ensure_dir(path: &Path) -> Result<()> {
    let path_buf = path.to_path_buf();
    fs::create_dir_all(path)
        .await
        .map_err(|e| IoError::CreateDir(e, path_buf).into())
}
