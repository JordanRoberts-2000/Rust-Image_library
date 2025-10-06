use {
    crate::{Image, ImageSrc, Result, WithSrc},
    fs_ext::{dir, file},
    std::{io, path::Path},
};

impl Image {
    pub fn save_to_folder(&self, folder_path: impl AsRef<Path>) -> Result<()> {
        dir::assert_exists(&folder_path).with_src(self.src())?;

        let path = folder_path.as_ref().join(self.file_name());

        file::atomic::overwrite(&path, |file| {
            self.encode(file, self.format())
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        })
        .with_src(self.src())?;

        if self.config.remove_source {
            if let ImageSrc::File(path) = &self.src {
                file::trash_or_remove(path).with_src(self.src())?;
            }
        }

        Ok(())
    }
}
