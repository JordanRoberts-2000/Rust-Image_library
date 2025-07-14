use crate::{image::r#async::ImageData, Image, ImageError, InternalError, Result};

use tokio::task::spawn_blocking;

impl Image {
    pub async fn blurhash(&self) -> Result<String> {
        self.apply_transforms().await?;

        let (pixels, width, height) = {
            let state = self.state.read().await;

            let decoded = match &state.data {
                ImageData::Decoded(img) => img.clone(),
                _ => {
                    return Err(InternalError::DecodingInvariantViolatedBeforeDecodeMatch.into());
                }
            };

            (
                decoded.to_rgba8().into_vec(),
                decoded.width(),
                decoded.height(),
            )
        };

        spawn_blocking(move || {
            blurhash::encode(4, 3, width, height, &pixels).map_err(ImageError::BlurHash)
        })
        .await
        .map_err(ImageError::TaskJoinError)?
    }
}
