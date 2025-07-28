use {
    crate::{blocking::dependencies::repos::fs::ensure_dir, IoError, Result, ValidationError},
    std::path::Path,
    tempfile::{Builder, NamedTempFile},
};

pub fn atomic_write<F>(path: &Path, write_fn: F) -> Result<()>
where
    F: FnOnce(&mut NamedTempFile) -> Result<()>,
{
    let parent = path.parent().ok_or_else(|| ValidationError::MissingParent(path.to_path_buf()))?;
    ensure_dir(parent)?;

    let mut temp = create_temp_file(parent)?;
    write_fn(&mut temp)?;

    temp.persist(path).map_err(|e| IoError::PersistTempFile(e.error, path.to_path_buf()))?;

    Ok(())
}

fn create_temp_file(parent: &Path) -> Result<NamedTempFile> {
    NamedTempFile::new()
        .or_else(|_| Builder::new().prefix(".").suffix(".tmp").tempfile_in(parent))
        .map_err(|e| IoError::CreateTempFile(e, parent.to_path_buf()).into())
}
