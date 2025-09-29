use {
    crate::{blocking::Images, Result},
    std::path::Path,
};

impl Images {
    pub(crate) fn to_tar(&self, path: &Path) -> Result<()> {
        Ok(())
    }
}
