use {
    crate::{IoError, Result},
    std::{fs, path::Path},
};

pub fn ensure_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path).map_err(|e| IoError::CreateDir(e, path.to_path_buf()).into())
}
