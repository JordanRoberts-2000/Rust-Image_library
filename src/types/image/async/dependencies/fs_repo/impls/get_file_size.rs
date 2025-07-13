use {
    crate::{IoError, Result},
    std::path::Path,
    tokio::fs,
};

pub async fn get_file_size(path: &Path) -> Result<u64> {
    let path_buf = path.to_path_buf();
    let metadata = fs::metadata(path)
        .await
        .map_err(|e| IoError::MetaData(e, path_buf.clone()))?;
    Ok(metadata.len())
}
