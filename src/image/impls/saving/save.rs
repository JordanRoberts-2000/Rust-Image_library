use crate::{Image, Result};

impl Image {
    pub fn save(&mut self) -> Result<()> {
        let ext = self.format.extention();
        let path = self
            .config
            .output_dir
            .join(format!("{}.{}", self.build_file_name(), ext));

        self.apply_transforms()?;
        self.atomic_save(&path)?;

        Ok(())
    }
}
