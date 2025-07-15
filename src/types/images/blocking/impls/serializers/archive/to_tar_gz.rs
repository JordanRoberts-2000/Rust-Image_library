use std::path::Path;

use crate::{blocking::Images, Result};

impl Images {
    pub(crate) fn to_tar_gz(&self, path: &Path) -> Result<()> {
        Ok(())
    }
}
