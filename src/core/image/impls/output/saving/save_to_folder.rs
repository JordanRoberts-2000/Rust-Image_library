use {
    crate::{ErrorKind, Image, ImageSrc, Result, ResultCtx},
    fs_ext::fsx,
    std::{io, path::Path},
};

impl Image {
    pub fn save_to_folder(&self, folder_path: impl AsRef<Path>) -> Result<()> {
        fsx::dir::assert_exists(&folder_path).ctx(ErrorKind::Save, self.error_src())?;

        let path = folder_path.as_ref().join(self.file_name());

        fsx::file::atomic::overwrite(&path, |file| {
            self.encode(file, self.format())
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        })
        .ctx(ErrorKind::Save, self.error_src())?;

        if self.config.remove_source {
            if let ImageSrc::File(path) = &self.src {
                fsx::file::trash_or_remove(path).ctx(ErrorKind::Save, self.error_src())?;
            }
        }

        Ok(())
    }
}
