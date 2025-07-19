use std::path::Path;

use crate::{Result, ValidationError};

pub fn check_existing_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(ValidationError::PathNotFound(path.to_path_buf()).into());
    }
    if !path.is_dir() {
        return Err(ValidationError::NotADirectory(path.to_path_buf()).into());
    }

    Ok(())
}
