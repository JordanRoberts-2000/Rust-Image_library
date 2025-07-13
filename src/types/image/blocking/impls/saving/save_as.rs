use std::path::Path;

use crate::{
    blocking::Image,
    image::{
        blocking::{dependencies::FsRepo, traits::FsRepoOps},
        enums::ImageSrc,
    },
    ImageError, ImageFormat, Result,
};

impl Image {
    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.save_as_internal(path.as_ref(), &FsRepo)
    }

    fn save_as_internal(&mut self, path: &Path, fs: &impl FsRepoOps) -> Result<()> {
        let mut path = path.to_path_buf();

        if path.extension().is_none() {
            let ext = self.format.extention();
            path.set_extension(ext);
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| ImageError::ExtensionMissing(path.to_path_buf()))?;

        let format = ImageFormat::try_from(ext)
            .map_err(|_| ImageError::InvalidExtension(ext.to_string()))?;

        self.apply_transforms()?;
        self.atomic_save(&path, format, fs)?;

        if self.config.remove_source {
            if let ImageSrc::File(path) = &self.src {
                fs.trash_file(path)?;
            }
        }

        Ok(())
    }
}
