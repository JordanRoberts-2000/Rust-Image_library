use {
    crate::{
        image::{
            r#async::ImageData,
            utils::{resolve_avif_config, to_rgba8_vec},
        },
        CompressionType, Image, ImageError, InternalError, Result,
    },
    ravif::{EncodedImage, Encoder, Img},
    tokio::{io::AsyncWrite, task::spawn_blocking},
};

impl Image {
    pub async fn encode_avif<W>(&self, writer: W) -> Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        let (decoded, config, id) = {
            let state = self.state.read().await;

            if state.config.compression == CompressionType::Lossless {
                log::warn!("Lossless AVIF compression is not supported; falling back to lossy.");
            }

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

        let encoded_data = spawn_blocking(move || {
            let (width, height) = decoded.dimensions();
            let (quality, speed, alpha_quality) = resolve_avif_config(&config);

            let pixels = to_rgba8_vec(&decoded);
            let img_ref = Img::new(pixels.as_slice(), width as usize, height as usize);

            let encoder = Encoder::new()
                .with_quality(quality)
                .with_speed(speed)
                .with_alpha_quality(alpha_quality);

            encoder
                .encode_rgba(img_ref)
                .map(|e: EncodedImage| e.avif_file)
                .map_err(|err| ImageError::AvifEncoding { err, id })
        })
        .await
        .map_err(ImageError::TaskJoinError)??;

        Self::write_encoded(writer, encoded_data, self.describe_source()).await
    }
}
