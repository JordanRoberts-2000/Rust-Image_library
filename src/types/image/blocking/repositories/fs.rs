use {
    crate::{image::blocking::traits::FsOps, IoError, Result, ValidationError},
    std::{fs, path::Path},
};

pub struct FsRepo;

impl FsOps for FsRepo {
    fn ensure_existing_file(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(ValidationError::PathNotFound(path.to_path_buf()).into());
        }
        if !path.is_file() {
            return Err(ValidationError::NotAFile(path.to_path_buf()).into());
        }

        Ok(())
    }

    fn get_file_size(&self, path: &Path) -> Result<u64> {
        let metadata = fs::metadata(path).map_err(|e| IoError::MetaData(e, path.to_path_buf()))?;
        Ok(metadata.len())
    }
}
