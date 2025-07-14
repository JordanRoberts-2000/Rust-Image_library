#[cfg(feature = "progressive-jpeg")]
use mozjpeg::{ColorSpace, Compress, ScanMode};

use crate::Image;

#[cfg(feature = "progressive-jpeg")]
use {
    crate::{constants::DEFAULT_JPEG_QUALITY, ImageError, InternalError, Result},
    tokio::{io::AsyncWrite, task::spawn_blocking},
};

impl Image {
    #[cfg(feature = "progressive-jpeg")]
    pub async fn encode_progressive_jpeg<W>(&self, writer: W) -> Result<()>
    where
        W: AsyncWrite + Unpin + Send + 'static,
    {
        let (rgb8, config, id) = {
            use crate::image::r#async::ImageData;

            let state = self.state.read().await;
            let img = match &state.data {
                ImageData::Decoded(img) => img.clone(),
                _ => {
                    return Err(InternalError::DecodingInvariantViolatedBeforeDecodeMatch.into());
                }
            };

            (img.to_rgb8(), state.config.clone(), self.describe_source())
        };

        let jpeg_data = spawn_blocking(move || {
            let (width, height) = rgb8.dimensions();
            let data = rgb8.as_raw();

            let quality = config
                .jpeg
                .as_ref()
                .map(|cfg| cfg.quality)
                .or_else(|| config.quality.map(|q| q as u8))
                .unwrap_or(DEFAULT_JPEG_QUALITY);

            let mut comp = Compress::new(ColorSpace::JCS_RGB);
            comp.set_size(width as usize, height as usize);
            comp.set_quality(quality as f32);
            comp.set_scan_optimization_mode(ScanMode::AllComponentsTogether);
            comp.set_progressive_mode();

            let mut comp_writer =
                comp.start_compress(Vec::new())
                    .map_err(|e| ImageError::JpegCompressionStart {
                        source: e,
                        id: id.clone(),
                    })?;

            comp_writer
                .write_scanlines(data)
                .map_err(|e| ImageError::JpegWriteScanlines {
                    source: e,
                    id: id.clone(),
                })?;

            comp_writer
                .finish()
                .map_err(|e| ImageError::JpegCompressionFinish {
                    source: e,
                    id: id.clone(),
                })
        })
        .await
        .map_err(ImageError::TaskJoinError)??;

        Self::write_encoded(writer, jpeg_data, self.describe_source()).await
    }
}
