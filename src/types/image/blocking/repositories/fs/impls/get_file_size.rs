use std::{fs, path::Path};

use crate::{IoError, Result};

pub fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = fs::metadata(path).map_err(|e| IoError::MetaData(e, path.to_path_buf()))?;
    Ok(metadata.len())
}
