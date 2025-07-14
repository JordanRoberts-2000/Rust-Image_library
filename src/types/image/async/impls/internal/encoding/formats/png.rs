use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use tokio::{io::AsyncWrite, task::spawn_blocking};

use crate::{image::r#async::ImageData, Image, ImageError, ImageFormat, InternalError, Result};

impl Image {
    pub async fn encode_png<W>(&self, writer: W) -> Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        let (rgb8, id) = {
            let state = self.state.read().await;

            let img = match &state.data {
                ImageData::Decoded(img) => img.clone(),
                _ => {
                    return Err(InternalError::DecodingInvariantViolatedBeforeDecodeMatch.into());
                }
            };

            (img.to_rgb8(), self.describe_source())
        };

        let encoded = spawn_blocking(move || {
            let mut buffer = Vec::new();
            let (width, height) = rgb8.dimensions();

            let encoder = PngEncoder::new(&mut buffer);
            encoder
                .write_image(&rgb8, width, height, ColorType::Rgb8.into())
                .map_err(|e| ImageError::Encoding {
                    source: e,
                    id: id.clone(),
                    format: ImageFormat::Png,
                })?;

            Ok::<_, ImageError>(buffer)
        })
        .await
        .map_err(ImageError::TaskJoinError)??;

        Self::write_encoded(writer, encoded, self.describe_source()).await
    }
}
