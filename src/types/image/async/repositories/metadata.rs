use {image::ImageReader, mockall::automock, std::path::Path};

use crate::{ImageError, ImageFormat};

#[automock]
pub trait AsyncMetadataRepo {
    async fn from_path(&self, path: &Path) -> Result<(ImageFormat, u32, u32), ImageError>;
}

pub struct DefaultAsyncMetadataRepo;

impl DefaultAsyncMetadataRepo {
    pub fn new() -> Self {
        Self {}
    }
}

impl AsyncMetadataRepo for DefaultAsyncMetadataRepo {
    async fn from_path(&self, path: &Path) -> Result<(ImageFormat, u32, u32), ImageError> {
        let path = path.to_path_buf();

        tokio::task::spawn_blocking(move || {
            let reader = ImageReader::open(&path).map_err(|e| ImageError::Open {
                source: e,
                path: path.clone(),
            })?;

            let format =
                ImageFormat::try_from(reader.format().ok_or_else(|| ImageError::UnknownFormat)?)?;

            let (width, height) = reader
                .into_dimensions()
                .map_err(ImageError::DimensionsFailed)?;

            Ok((format, width, height))
        })
        .await
        .map_err(|join_error| ImageError::TaskJoinError(join_error))?
    }
}
