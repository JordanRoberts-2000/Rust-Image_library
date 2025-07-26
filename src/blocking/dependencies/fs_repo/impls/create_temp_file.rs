use {
    crate::{IoError, Result},
    std::path::Path,
    tempfile::{Builder, NamedTempFile},
};

pub fn create_temp_file(parent: &Path) -> Result<NamedTempFile> {
    NamedTempFile::new()
        .or_else(|_| Builder::new().prefix(".").suffix(".tmp").tempfile_in(parent))
        .map_err(|e| IoError::CreateTempFile(e, parent.to_path_buf()).into())
}
