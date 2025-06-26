use std::path::{Path, PathBuf};

use crate::ValidationError;

pub fn existing_dir(path: impl AsRef<Path>) -> Result<PathBuf, ValidationError> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(ValidationError::PathNotFound(path.to_path_buf()));
    }
    if !path.is_dir() {
        return Err(ValidationError::NotADirectory(path.to_path_buf()));
    }

    Ok(path.to_path_buf())
}
