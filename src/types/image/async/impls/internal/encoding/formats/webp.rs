use tokio::{io::AsyncWrite, task::spawn_blocking};

use crate::{
    image::{
        r#async::ImageData,
        utils::{encode_webp_data, resolve_webp_config},
    },
    Image, ImageError, InternalError, Result,
};

impl Image {
    pub async fn encode_webp<W>(&self, writer: W) -> Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        let (rgba_image, config, id) = {
            let state = self.state.read().await;

            let decoded = match &state.data {
                ImageData::Decoded(img) => img.clone(),
                _ => {
                    return Err(InternalError::DecodingInvariantViolatedBeforeDecodeMatch.into());
                }
            };

            (
                decoded.to_rgba8(),
                state.config.clone(),
                self.describe_source(),
            )
        };

        let webp_data = spawn_blocking(move || {
            let (width, height) = rgba_image.dimensions();
            let (lossless, quality) = resolve_webp_config(&config);
            encode_webp_data(&rgba_image, width, height, lossless, quality, &id)
        })
        .await
        .map_err(ImageError::TaskJoinError)??;

        Self::write_encoded(writer, webp_data, self.describe_source()).await
    }
}
