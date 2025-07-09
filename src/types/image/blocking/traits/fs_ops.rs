use std::path::Path;

use crate::ImageError;

pub trait FsOps {
    fn ensure_existing_file(&self, path: &Path) -> Result<(), ImageError>;
}
