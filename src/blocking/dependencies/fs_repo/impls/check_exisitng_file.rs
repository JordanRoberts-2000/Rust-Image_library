use {
    crate::{Result, ValidationError},
    std::path::Path,
};

pub fn check_existing_file(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(ValidationError::PathNotFound(path.to_path_buf()).into());
    }
    if !path.is_file() {
        return Err(ValidationError::NotAFile(path.to_path_buf()).into());
    }

    Ok(())
}
