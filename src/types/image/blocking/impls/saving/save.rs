use crate::{
    blocking::{dependencies::FsRepo, traits::FsRepoOps, Image},
    image::enums::ImageSrc,
    Result,
};

impl Image {
    pub fn save(&mut self) -> Result<()> {
        self.save_internal(&FsRepo)
    }

    fn save_internal(&mut self, fs: &impl FsRepoOps) -> Result<()> {
        let path = self.config.output_dir.join(self.file_name());

        self.atomic_save(&path, self.format, fs)?;

        if self.config.remove_source {
            if let ImageSrc::File(path) = &self.src {
                fs.trash_file(path)?;
            }
        }

        Ok(())
    }
}
