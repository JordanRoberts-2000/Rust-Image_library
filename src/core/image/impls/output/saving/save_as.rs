use {
    crate::{image::ImageSrc, Image, ImageError, ImageFormat, Result},
    fs_ext::fsx::file,
    std::{io, path::Path},
};

impl Image {
    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let mut path = path.as_ref().to_path_buf();

        let ext = match path.extension() {
            None => {
                let ext = self.format().extension();
                path.set_extension(ext);
                ext
            }
            Some(os_str) => {
                os_str.to_str().ok_or_else(|| ImageError::ExtensionMissing(path.to_path_buf()))?
            }
        };

        let format = ImageFormat::try_from(ext)
            .map_err(|_| ImageError::InvalidExtension(ext.to_string()))?;

        file::atomic::overwrite(&path, |file| {
            self.encode(file, format).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        })?;

        if self.config.remove_source {
            if let ImageSrc::File(path) = &self.src {
                file::trash_or_remove(path)?;
            }
        }

        Ok(())
    }
}
