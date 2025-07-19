use std::path::Path;

use tempfile::{Builder, NamedTempFile};

use crate::{IoError, Result};

pub fn create_temp_file(parent: &Path) -> Result<NamedTempFile> {
    NamedTempFile::new()
        .or_else(|_| {
            Builder::new()
                .prefix(".")
                .suffix(".tmp")
                .tempfile_in(parent)
        })
        .map_err(|e| IoError::CreateTempFile(e, parent.to_path_buf()).into())
}
