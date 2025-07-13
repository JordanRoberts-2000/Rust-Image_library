use {
    crate::{IoError, Result},
    std::path::Path,
    tempfile::{Builder, NamedTempFile},
    tokio::task::spawn_blocking,
};

pub async fn create_temp_file(parent: &Path) -> Result<NamedTempFile> {
    let parent = parent.to_path_buf();

    spawn_blocking(move || {
        NamedTempFile::new()
            .or_else(|_| {
                Builder::new()
                    .prefix(".")
                    .suffix(".tmp")
                    .tempfile_in(&parent)
            })
            .map_err(|e| IoError::CreateTempFile(e, parent.clone()).into())
    })
    .await
    .map_err(crate::ImageError::TaskJoinError)?
}
