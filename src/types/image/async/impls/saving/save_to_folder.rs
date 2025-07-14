use std::path::Path;

use crate::{
    image::{
        enums::ImageSrc,
        r#async::{dependencies::FsRepo, traits::FsRepoOps},
    },
    Image, Result,
};

impl Image {
    pub async fn save_to_folder(&mut self, folder_path: impl AsRef<Path>) -> Result<()> {
        self.save_to_folder_internal(folder_path.as_ref(), &FsRepo)
            .await
    }

    async fn save_to_folder_internal(
        &mut self,
        folder_path: &Path,
        fs: &impl FsRepoOps,
    ) -> Result<()> {
        fs.check_existing_dir(folder_path).await?;

        let (format, remove_source) = {
            let state = self.state.read().await;
            (state.format, state.config.remove_source)
        };

        let ext = format.extention();
        let path = folder_path.join(format!("{}.{}", self.build_file_name().await, ext));

        self.apply_transforms().await?;
        self.atomic_save(&path, format, fs).await?;

        if remove_source {
            if let ImageSrc::File(path) = &self.src {
                fs.trash_file(path).await?;
            }
        }

        Ok(())
    }
}
