use {
    crate::{image::ImageSrc, Image, Result},
    fs_ext::fsx::file,
    std::io,
};

impl Image {
    pub fn save(&self) -> Result<()> {
        let path = self.config.output_dir.join(self.file_name());

        file::atomic::overwrite(&path, |file| {
            self.encode(file, self.format()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        })?;

        if self.config.remove_source {
            if let ImageSrc::File(path) = &self.src {
                file::trash_or_remove(path)?;
            }
        }

        Ok(())
    }
}
