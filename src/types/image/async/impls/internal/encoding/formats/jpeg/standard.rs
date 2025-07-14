use crate::{
    constants::DEFAULT_JPEG_QUALITY, image::r#async::ImageData, Image, ImageError, ImageFormat,
    InternalError, Result,
};

use {
    image::{codecs::jpeg::JpegEncoder, ExtendedColorType},
    tokio::{io::AsyncWrite, task::spawn_blocking},
};

impl Image {
    pub async fn encode_jpeg<W>(&self, writer: W) -> Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        #[cfg(feature = "progressive-jpeg")]
        {
            if self
                .state
                .read()
                .await
                .config
                .jpeg
                .as_ref()
                .map_or(false, |cfg| cfg.progressive)
            {
                return self.encode_progressive_jpeg(writer).await;
            }
        }

        let (rgb8, quality, id) = {
            let state = self.state.read().await;
            let img = match &state.data {
                ImageData::Decoded(img) => img.clone(),
                _ => {
                    return Err(InternalError::DecodingInvariantViolatedBeforeDecodeMatch.into());
                }
            };

            let quality = state
                .config
                .jpeg
                .as_ref()
                .map(|cfg| cfg.quality)
                .or_else(|| state.config.quality.map(|q| q as u8))
                .unwrap_or(DEFAULT_JPEG_QUALITY);

            (img.to_rgb8(), quality, self.describe_source())
        };

        let encoded_data = spawn_blocking(move || {
            let (width, height) = rgb8.dimensions();
            let mut buffer = Vec::new();

            let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);
            encoder
                .encode(rgb8.as_raw(), width, height, ExtendedColorType::Rgb8)
                .map_err(|e| ImageError::Encoding {
                    source: e,
                    id: id.clone(),
                    format: ImageFormat::Jpeg,
                })?;

            Ok::<_, ImageError>(buffer)
        })
        .await
        .map_err(ImageError::TaskJoinError)??;

        Self::write_encoded(writer, encoded_data, self.describe_source()).await
    }
}
