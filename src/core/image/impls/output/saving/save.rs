use {
    crate::{Image, ImageSrc, Result, WithSrc},
    fs_ext::file,
};

impl Image {
    pub fn save(&self) -> Result<()> {
        let path = self.config.output_dir.join(self.file_name());

        file::atomic::overwrite(&path, |file| self.encode(file, self.format()))
            .with_src(self.error_src())?;

        if self.config.remove_source {
            if let ImageSrc::File(path) = &self.src {
                file::trash_or_remove(path).with_src(self.error_src())?;
            }
        }

        Ok(())
    }
}
