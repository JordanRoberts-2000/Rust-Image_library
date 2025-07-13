use {
    crate::{ImageError, IoError, Result, ValidationError},
    std::path::Path,
    tokio::{fs, task::spawn_blocking},
};

pub async fn trash_file(path: &Path) -> Result<()> {
    let path_buf = path.to_path_buf();

    let metadata = fs::metadata(path)
        .await
        .map_err(|_| ValidationError::PathNotFound(path_buf.clone()))?;

    if !metadata.is_file() {
        return Err(ValidationError::NotAFile(path_buf.clone()).into());
    }

    let trash_result = spawn_blocking({
        let path = path.to_path_buf();
        move || trash::delete(&path)
    })
    .await
    .map_err(ImageError::TaskJoinError)?;

    if let Err(trash_err) = trash_result {
        log::warn!(
            "Failed to trash '{}'. Falling back to permanent delete. err: {:?}",
            path.display(),
            trash_err
        );

        fs::remove_file(path)
            .await
            .map_err(|e| IoError::DeleteFile(e, path_buf))?;
    }

    Ok(())
}
