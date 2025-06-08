use std::path::Path;

use crate::{IoError, Result, ValidationError};

pub fn trash_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(ValidationError::PathNotFound(path.to_path_buf()).into());
    }

    if !path.is_file() {
        return Err(ValidationError::NotAFile(path.to_path_buf()).into());
    }

    if let Err(trash_err) = trash::delete(path) {
        log::warn!(
            "Failed to trash '{}'. Falling back to permanent delete. err: {:?}",
            path.display(),
            trash_err
        );

        std::fs::remove_file(path).map_err(|e| IoError::DeleteFile(e, path.to_path_buf()))?;
    }

    Ok(())
}
