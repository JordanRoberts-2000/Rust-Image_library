use crate::{
    image::{
        enums::ImageSrc,
        r#async::{dependencies::FsRepo, traits::FsRepoOps},
    },
    Image, Result,
};

impl Image {
    pub async fn save(&mut self) -> Result<()> {
        self.save_internal(&FsRepo).await
    }

    async fn save_internal(&mut self, fs: &impl FsRepoOps) -> Result<()> {
        let (format, config) = {
            let state = self.state.read().await;
            (state.format, state.config.clone())
        };

        let ext = format.extention();
        let path = config
            .output_dir
            .join(format!("{}.{}", self.build_file_name().await, ext));

        self.apply_transforms().await?;
        self.atomic_save(&path, format, fs).await?;

        if config.remove_source {
            if let ImageSrc::File(path) = &self.src {
                fs.trash_file(path).await?;
            }
        }

        Ok(())
    }
}
