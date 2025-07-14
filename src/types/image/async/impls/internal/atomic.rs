use std::path::Path;

use crate::{image::r#async::traits::FsRepoOps, Image, ImageError, ImageFormat, IoError, Result};

impl Image {
    pub async fn atomic_save(
        &mut self,
        path: &Path,
        format: ImageFormat,
        fs: &impl FsRepoOps,
    ) -> Result<()> {
        let parent = path
            .parent()
            .ok_or_else(|| ImageError::MissingParent(path.to_path_buf()))?;
        fs.ensure_dir(parent).await?;

        let temp_file = fs.create_temp_file(parent).await?;

        let file = tokio::fs::File::create(temp_file.path())
            .await
            .map_err(|e| IoError::CreateFile(e, temp_file.path().to_path_buf()))?;

        self.encode(file, format).await?;

        fs.persist_temp_file(temp_file, path).await?;

        Ok(())
    }
}
