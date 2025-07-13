use crate::{
    blocking::Image,
    image::{
        blocking::{traits::FsRepoOps, FsRepo},
        enums::ImageSrc,
    },
    Result,
};

impl Image {
    pub fn save(&mut self) -> Result<()> {
        self.save_internal(&FsRepo)
    }

    fn save_internal(&mut self, fs: &impl FsRepoOps) -> Result<()> {
        let ext = self.format.extention();
        let path = self
            .config
            .output_dir
            .join(format!("{}.{}", self.build_file_name(), ext));

        self.apply_transforms()?;
        self.atomic_save(&path, self.format, fs)?;

        if self.config.remove_source {
            if let ImageSrc::File(path) = &self.src {
                fs.trash_file(path)?;
            }
        }

        Ok(())
    }
}
