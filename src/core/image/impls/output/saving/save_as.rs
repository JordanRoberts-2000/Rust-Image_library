use {
    crate::{ErrorKind, Image, ImageFormat, ImageSrc, InnerError, Result, ResultCtx},
    fs_ext::file,
    std::{io, path::Path},
};

impl Image {
    pub fn save_as(&self, path: impl AsRef<Path>) -> Result<()> {
        let mut path = path.as_ref().to_path_buf();

        let ext = match path.extension() {
            None => {
                let ext = self.format().extension();
                path.set_extension(ext);
                ext
            }
            Some(os_str) => os_str
                .to_str()
                .ok_or_else(|| InnerError::ExtensionMissing(path.to_path_buf()))
                .ctx(ErrorKind::Save, self.error_src())?,
        };

        let format = ImageFormat::try_from(ext)
            .map_err(|_| InnerError::InvalidExtension(ext.to_string()))
            .ctx(ErrorKind::Save, self.error_src())?;

        file::atomic::overwrite(&path, |file| {
            self.encode(file, format)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        })
        .ctx(ErrorKind::Save, self.error_src())?;

        if self.config.remove_source {
            if let ImageSrc::File(path) = &self.src {
                file::trash_or_remove(path).ctx(ErrorKind::Save, self.error_src())?;
            }
        }

        Ok(())
    }
}
