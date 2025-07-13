use {
    crate::{ImageError, IoError, Result},
    std::{fs::File, path::Path},
    tempfile::NamedTempFile,
    tokio::task::spawn_blocking,
};

pub async fn persist_temp_file(temp: NamedTempFile, path: &Path) -> Result<File> {
    let path = path.to_path_buf();

    spawn_blocking(move || {
        temp.persist(&path)
            .map_err(|e| IoError::PersistTempFile(e.error, path.clone()).into())
    })
    .await
    .map_err(ImageError::TaskJoinError)?
}
