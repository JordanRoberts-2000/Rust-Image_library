use std::path::Path;

use crate::{utils::validation::existing_dir, Image, Result};

impl Image {
    pub fn save_to_folder(&mut self, folder_path: impl AsRef<Path>) -> Result<()> {
        existing_dir(&folder_path)?;

        let ext = self.format.extention();
        let path = folder_path
            .as_ref()
            .join(format!("{}.{}", self.build_file_name(), ext));

        self.apply_transforms()?;
        self.atomic_save(&path)?;

        Ok(())
    }
}
