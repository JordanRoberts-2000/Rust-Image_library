use crate::{image::r#async::ImageData, Image, ImageError, InternalError, Result};

use tokio::task::spawn_blocking;

impl Image {
    pub async fn to_raw_pixels(&self) -> Result<Vec<u8>> {
        self.apply_transforms().await?;

        let decoded = {
            let state = self.state.read().await;
            match &state.data {
                ImageData::Decoded(img) => img.clone(),
                _ => {
                    return Err(InternalError::DecodingInvariantViolatedBeforeDecodeMatch.into());
                }
            }
        };

        let pixels = spawn_blocking(move || Ok::<_, ImageError>(decoded.to_rgba8().into_raw()))
            .await
            .map_err(ImageError::TaskJoinError)??;

        Ok(pixels)
    }
}
