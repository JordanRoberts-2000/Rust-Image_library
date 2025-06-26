use crate::{utils::fs::trash_file, Image, ImageSrc, Result};

impl Image {
    pub fn save(&mut self) -> Result<()> {
        let ext = self.format.extention();
        let path = self
            .config
            .output_dir
            .join(format!("{}.{}", self.build_file_name(), ext));

        self.apply_transforms()?;
        self.atomic_save(&path)?;

        if self.config.remove_source {
            if let ImageSrc::File(path) = &self.src {
                trash_file(path)?;
            }
        }

        Ok(())
    }
}
