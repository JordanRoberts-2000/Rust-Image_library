use {mockall::automock, std::path::Path};

use crate::{utils::validation, ImageError};

#[automock]
pub trait AsyncValidationRepo: Send + Sync {
    async fn ensure_existing_image_file(&self, path: &Path) -> Result<(), ImageError>;
}

pub struct DefaultAsyncValidationRepo;

impl DefaultAsyncValidationRepo {
    pub fn new() -> Self {
        Self {}
    }
}

impl AsyncValidationRepo for DefaultAsyncValidationRepo {
    async fn ensure_existing_image_file(&self, path: &Path) -> Result<(), ImageError> {
        let path = path.to_path_buf();

        tokio::task::spawn_blocking(move || validation::ensure_existing_image_file(&path))
            .await
            .map_err(ImageError::TaskJoinError)?;

        Ok(())
    }
}
