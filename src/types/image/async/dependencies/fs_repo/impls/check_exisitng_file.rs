use {
    crate::{IoError, Result, ValidationError},
    std::path::Path,
    tokio::fs,
};

pub async fn check_existing_file(path: &Path) -> Result<()> {
    let metadata = fs::metadata(path)
        .await
        .map_err(|e| IoError::MetaData(e, path.to_path_buf()))?;

    if !metadata.is_file() {
        return Err(ValidationError::NotAFile(path.to_path_buf()).into());
    }

    Ok(())
}
