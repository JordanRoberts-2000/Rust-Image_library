use {
    crate::{
        blocking::{dependencies::FsRepo, traits::FsRepoOps, Image},
        image::enums::ImageSrc,
        Result,
    },
    std::path::Path,
};

impl Image {
    pub fn save_to_folder(&mut self, folder_path: impl AsRef<Path>) -> Result<()> {
        self.save_to_folder_internal(folder_path.as_ref(), &FsRepo)
    }

    fn save_to_folder_internal(&mut self, folder_path: &Path, fs: &impl FsRepoOps) -> Result<()> {
        fs.check_existing_dir(&folder_path)?;

        let path = folder_path.join(self.file_name());

        self.atomic_save(&path, self.format, fs)?;

        if self.config.remove_source {
            if let ImageSrc::File(path) = &self.src {
                fs.trash_file(path)?;
            }
        }

        Ok(())
    }
}
