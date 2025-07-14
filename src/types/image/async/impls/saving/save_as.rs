use std::path::Path;

use crate::{
    image::{
        enums::ImageSrc,
        r#async::{dependencies::FsRepo, traits::FsRepoOps},
    },
    Image, ImageError, ImageFormat, Result,
};

impl Image {
    pub async fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.save_as_internal(path.as_ref(), &FsRepo).await
    }

    async fn save_as_internal(&mut self, path: &Path, fs: &impl FsRepoOps) -> Result<()> {
        let mut path = path.to_path_buf();

        let (default_format, remove_source) = {
            let state = self.state.read().await;
            (state.format, state.config.remove_source)
        };

        if path.extension().is_none() {
            let ext = default_format.extention();
            path.set_extension(ext);
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| ImageError::ExtensionMissing(path.to_path_buf()))?;

        let format = ImageFormat::try_from(ext)
            .map_err(|_| ImageError::InvalidExtension(ext.to_string()))?;

        self.apply_transforms().await?;
        self.atomic_save(&path, format, fs).await?;

        if remove_source {
            if let ImageSrc::File(path) = &self.src {
                fs.trash_file(path).await?;
            }
        }

        Ok(())
    }
}
