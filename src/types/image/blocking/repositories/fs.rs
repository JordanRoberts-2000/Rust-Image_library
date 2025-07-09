use std::path::Path;

use crate::{image::blocking::traits::FsOps, ImageError, ValidationError};

pub struct FsRepo;

impl FsOps for FsRepo {
    fn ensure_existing_file(&self, path: &Path) -> Result<(), ImageError> {
        if !path.exists() {
            return Err(ValidationError::PathNotFound(path.to_path_buf()).into());
        }
        if !path.is_file() {
            return Err(ValidationError::NotAFile(path.to_path_buf()).into());
        }

        Ok(())
    }
}
