use std::path::Path;

use crate::{blocking::Images, Result};

impl Images {
    pub(crate) fn to_tar(&self, path: &Path) -> Result<()> {
        Ok(())
    }
}
