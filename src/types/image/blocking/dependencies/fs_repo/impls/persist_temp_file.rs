use std::{fs::File, path::Path};

use tempfile::NamedTempFile;

use crate::{IoError, Result};

pub fn persist_temp_file(temp: NamedTempFile, path: &Path) -> Result<File> {
    temp.persist(path)
        .map_err(|e| IoError::PersistTempFile(e.error, path.to_path_buf()).into())
}
